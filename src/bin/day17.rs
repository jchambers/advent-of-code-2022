use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

const CAVE_WIDTH: usize = 7;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let mut cave = Cave::from_str(fs::read_to_string(path)?.as_str())?;

        for _ in 0..2022 {
            cave.add_rock();
        }

        println!(
            "Tower height after adding 2022 rocks: {}",
            cave.tower_height()
        );

        Ok(())
    } else {
        Err("Usage: day17 INPUT_FILE_PATH".into())
    }
}

struct Cave {
    spaces: Vec<Space>,

    rocks: [Rock; 5],
    next_rock: usize,

    jet_pattern: Vec<Jet>,
    next_jet: usize,
}

impl FromStr for Cave {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let jet_pattern = string.chars()
            .map(|c| match c {
                '<' => Ok(Jet::Left),
                '>' => Ok(Jet::Right),
                _ => Err("Unexpected character".into()),
            })
            .collect::<Result<_, Box<dyn Error>>>()?;

        Ok(Cave {
            spaces: vec![],

            rocks: [
                Rock::h_bar(),
                Rock::cross(),
                Rock::corner(),
                Rock::v_bar(),
                Rock::square(),
            ],
            next_rock: 0,

            jet_pattern,
            next_jet: 0,
        })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.spaces
            .chunks_exact(CAVE_WIDTH)
            .rev()
            .try_for_each(|row| {
                let row_string: String = row
                    .iter()
                    .map(|space| match space {
                        Space::Empty => '.',
                        Space::Rock => '#',
                    })
                    .collect();

                writeln!(f, "{}", row_string)
            })?;

        Ok(())
    }
}

impl Cave {
    fn add_rock(&mut self) {
        let mut position = (2, self.tower_height() + 3);

        let rock = self.rocks[self.next_rock].clone();
        self.next_rock = (self.next_rock + 1) % self.rocks.len();

        // Do we need to add rows to the cave?
        if self.cave_height() < position.1 + rock.height() {
            let additional_rows = position.1 + rock.height() - self.cave_height();
            self.spaces.append(&mut vec![Space::Empty; additional_rows * CAVE_WIDTH]);
        }

        loop {
            let jet = &self.jet_pattern[self.next_jet];
            self.next_jet = (self.next_jet + 1) % self.jet_pattern.len();

            let blocked_by_cave_wall = match jet {
                Jet::Left => position.0 == 0,
                Jet::Right => position.0 + rock.width() == CAVE_WIDTH,
            };

            position = if !blocked_by_cave_wall {
                let gusted_position = match jet {
                    Jet::Left => (position.0 - 1, position.1),
                    Jet::Right => (position.0 + 1, position.1),
                };

                if self.collides_with_rock(&rock, gusted_position) {
                    position
                } else {
                    gusted_position
                }
            } else {
                position
            };

            if position.1 == 0 {
                // We've reached the bottom
                self.fill_with_rock(&rock, position);
                break;
            } else {
                let position_after_fall = (position.0, position.1 - 1);

                if self.collides_with_rock(&rock, position_after_fall) {
                    // Falling would cause a collision, so settle the rock where it is
                    self.fill_with_rock(&rock, position);
                    break;
                } else {
                    position = position_after_fall;
                }
            }
        }
    }

    fn cave_height(&self) -> usize {
        self.spaces.len() / CAVE_WIDTH
    }

    fn tower_height(&self) -> usize {
        if self.spaces.is_empty() {
            0
        } else {
            self.spaces
                .chunks_exact(CAVE_WIDTH)
                .enumerate()
                .rev()
                .find(|(_, row)| row.iter().any(|space| matches!(space, Space::Rock)))
                .map(|(i, _)| i)
                .expect("Non-empty rows should have at least one rock space") + 1
        }
    }

    fn collides_with_rock(&self, rock: &Rock, position: (usize, usize)) -> bool {
        rock.filled_spaces
            .iter()
            .map(|(x, y)| (x + position.0, y + position.1))
            .any(|(x, y)| matches!(self.spaces[(y * CAVE_WIDTH) + x], Space::Rock))
    }

    fn fill_with_rock(&mut self, rock: &Rock, position: (usize, usize)) {
        rock.filled_spaces
            .iter()
            .map(|(x, y)| (x + position.0, y + position.1))
            .for_each(|(x, y)| self.spaces[(y * CAVE_WIDTH) + x] = Space::Rock);
    }
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum Space {
    Empty,
    Rock,
}

#[derive(Clone)]
struct Rock {
    filled_spaces: Vec<(usize, usize)>,
}

impl Rock {
    fn h_bar() -> Rock {
        Rock { filled_spaces: vec![(0, 0), (1, 0), (2, 0), (3, 0)] }
    }

    fn cross() -> Rock {
        Rock { filled_spaces: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)] }
    }

    fn corner() -> Rock {
        Rock { filled_spaces: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)] }
    }

    fn v_bar() -> Rock {
        Rock { filled_spaces: vec![(0, 0), (0, 1), (0, 2), (0, 3)] }
    }

    fn square() -> Rock {
        Rock { filled_spaces: vec![(0, 0), (1, 0), (0, 1), (1, 1)] }
    }

    fn height(&self) -> usize {
        *self.filled_spaces
            .iter()
            .map(|(_, y)| y)
            .max()
            .unwrap() as usize + 1
    }

    fn width(&self) -> usize {
        *self.filled_spaces
            .iter()
            .map(|(x, _)| x)
            .max()
            .unwrap() as usize + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PATTERN: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_tower_height() {
        let mut cave = Cave::from_str(TEST_PATTERN).unwrap();

        for _ in 0..2022 {
            cave.add_rock();
        }

        assert_eq!(3068, cave.tower_height());
    }
}
