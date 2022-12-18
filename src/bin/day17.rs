use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

const CAVE_WIDTH: usize = 7;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        for rocks in [2022, 1_000_000_000_000u64] {
            let cave = Cave::from_str(fs::read_to_string(path)?.as_str())?;

            println!(
                "Tower height after adding {} rocks: {}",
                rocks,
                cave.tower_height(rocks)
            );
        }

        Ok(())
    } else {
        Err("Usage: day17 INPUT_FILE_PATH".into())
    }
}

struct Cave {
    spaces: Vec<Space>,
    rocks_added: usize,

    rocks: [Rock; 5],
    next_rock: usize,

    jet_pattern: Vec<Jet>,
    next_jet: usize,
}

impl FromStr for Cave {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let jet_pattern = string
            .chars()
            .map(|c| match c {
                '<' => Ok(Jet::Left),
                '>' => Ok(Jet::Right),
                _ => Err("Unexpected character".into()),
            })
            .collect::<Result<_, Box<dyn Error>>>()?;

        Ok(Cave {
            spaces: vec![],
            rocks_added: 0,

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
    fn tower_height(mut self, rocks: u64) -> u64 {
        // Hypothesis: as we add rocks, we'll wind up with an irregular tower "base," but then
        // eventually settle into a repeating pattern. Here, we add rocks until we find a repeating
        // pattern, then figure out the base tower height, then figure out how long the repeating
        // cycle is.

        // We can't possibly have a cycle until (a) we've added at least one of each rock, and then
        // that cycle can't possibly be shorter than the height of the tallest individual rock.
        const MIN_CYCLE_HEIGHT: usize = 4;

        let (cycle_start, cycle_end) = 'find_cycle: loop {
            self.add_rock();

            let tower_height = self.current_tower_height();

            if self.rocks_added as u64 == rocks {
                return tower_height as u64;
            }

            if self.rocks_added as usize > self.rocks.len() && tower_height >= 2 * MIN_CYCLE_HEIGHT
            {
                for potential_cycle_height in MIN_CYCLE_HEIGHT..=tower_height / 2 {
                    let top = tower_height * CAVE_WIDTH;
                    let midpoint = top - (potential_cycle_height * CAVE_WIDTH);
                    let bottom = top - (2 * potential_cycle_height * CAVE_WIDTH);

                    if self.spaces[midpoint..top] == self.spaces[bottom..midpoint] {
                        // We've found a cycle! We know how tall it is, but not how many rocks went
                        // into it.
                        break 'find_cycle (midpoint, top);
                    }
                }
            }
        };

        // Now we know how tall the cycle is; let's figure out how many rocks go into that cycle.
        let rocks_added_before_repeat = self.rocks_added;

        loop {
            // The cycle can only reasonably repeat after adding a full round of rocks
            for _ in 0..self.rocks.len() {
                self.add_rock();

                if self.rocks_added as u64 == rocks {
                    return self.current_tower_height() as u64;
                }
            }

            let end = self.current_tower_height() * CAVE_WIDTH;
            let start = end - (cycle_end - cycle_start);

            if self.spaces[start..end] == self.spaces[cycle_start..cycle_end] {
                // The cycle has repeated!
                break;
            }
        }

        let rocks_per_cycle = self.rocks_added - rocks_added_before_repeat;

        // We now know enough to calculate the number of rocks per cycle, and we know we've got
        // three full cycles plus the initial bottom tower. That tells us how many rocks are in the
        // base tower and how tall the base tower is.
        let rocks_in_base_tower = self.rocks_added - (3 * rocks_per_cycle);
        let base_tower_height =
            self.current_tower_height() - (3 * (cycle_end - cycle_start) / CAVE_WIDTH);

        // This is MOST of what we need to calculate the full height of the tower. There may be a
        // few stragglers after the last cycle boundary; we'll just add those and see how the height
        // changes.
        let rocks_after_last_cycle = (rocks as usize - rocks_in_base_tower) % rocks_per_cycle;

        let height_from_rocks_after_last_cycle = {
            let initial_height = self.current_tower_height();

            for _ in 0..rocks_after_last_cycle {
                self.add_rock();
            }

            self.current_tower_height() - initial_height
        };

        // And, finally, putting it all together…
        let cycle_height = (cycle_end - cycle_start) / CAVE_WIDTH;
        let full_cycles = (rocks as usize - rocks_in_base_tower) / rocks_per_cycle;

        (base_tower_height + (full_cycles * cycle_height) + height_from_rocks_after_last_cycle)
            as u64
    }

    fn add_rock(&mut self) {
        let mut position = (2, self.current_tower_height() + 3);

        let rock = self.rocks[self.next_rock].clone();
        self.next_rock = (self.next_rock + 1) % self.rocks.len();

        // Do we need to add rows to the cave?
        if self.cave_height() < position.1 + rock.height() {
            let additional_rows = position.1 + rock.height() - self.cave_height();
            self.spaces
                .append(&mut vec![Space::Empty; additional_rows * CAVE_WIDTH]);
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

        self.rocks_added += 1;
    }

    fn cave_height(&self) -> usize {
        self.spaces.len() / CAVE_WIDTH
    }

    fn current_tower_height(&self) -> usize {
        if self.spaces.is_empty() {
            0
        } else {
            self.spaces
                .chunks_exact(CAVE_WIDTH)
                .enumerate()
                .rev()
                .find(|(_, row)| row.iter().any(|space| matches!(space, Space::Rock)))
                .map(|(i, _)| i)
                .expect("Non-empty rows should have at least one rock space")
                + 1
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

#[derive(Copy, Clone, Eq, PartialEq)]
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
        Rock {
            filled_spaces: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        }
    }

    fn cross() -> Rock {
        Rock {
            filled_spaces: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        }
    }

    fn corner() -> Rock {
        Rock {
            filled_spaces: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        }
    }

    fn v_bar() -> Rock {
        Rock {
            filled_spaces: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        }
    }

    fn square() -> Rock {
        Rock {
            filled_spaces: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn height(&self) -> usize {
        *self.filled_spaces.iter().map(|(_, y)| y).max().unwrap() as usize + 1
    }

    fn width(&self) -> usize {
        *self.filled_spaces.iter().map(|(x, _)| x).max().unwrap() as usize + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PATTERN: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_tower_height() {
        let cave = Cave::from_str(TEST_PATTERN).unwrap();

        assert_eq!(3068, cave.tower_height(2022));
    }
}
