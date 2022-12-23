use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{fs, iter};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let mut grove = Grove::from_str(fs::read_to_string(path)?.as_str())?;

        for _ in 0..10 {
            grove.advance_round();
        }

        println!(
            "Empty ground tiles after 10 rounds: {}",
            grove.empty_ground_tiles()
        );

        Ok(())
    } else {
        Err("Usage: day23 INPUT_FILE_PATH".into())
    }
}

struct Grove {
    elves: HashSet<(i32, i32)>,
    round: usize,
}

impl Grove {
    fn bounds(&self) -> ((i32, i32), (i32, i32)) {
        self.elves.iter().fold(
            ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
            |bounds, (x, y)| {
                (
                    (bounds.0 .0.min(*x), bounds.0 .1.min(*y)),
                    (bounds.1 .0.max(*x), bounds.1 .1.max(*y)),
                )
            },
        )
    }

    fn empty_ground_tiles(&self) -> u32 {
        let ((x_min, y_min), (x_max, y_max)) = self.bounds();
        let area = (x_min.abs_diff(x_max) + 1) * (y_min.abs_diff(y_max) + 1);

        area - self.elves.len() as u32
    }

    fn advance_round(&mut self) {
        const SEARCH_ORDER: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        let mut proposals: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for (x, y) in &self.elves {
            let mut has_neighbor = false;

            'neighbor_search: for neighbor_x in x - 1..=x + 1 {
                for neighbor_y in y - 1..=y + 1 {
                    // Ignore ourselves when searching for neighbors
                    if !(neighbor_x == *x && neighbor_y == *y)
                        && self.elves.contains(&(neighbor_x, neighbor_y))
                    {
                        has_neighbor = true;
                        break 'neighbor_search;
                    }
                }
            }

            if has_neighbor {
                for d in 0..SEARCH_ORDER.len() {
                    let direction = &SEARCH_ORDER[(d + self.round) % SEARCH_ORDER.len()];

                    let mut neighbors: Box<dyn Iterator<Item = (i32, i32)>> = match direction {
                        Direction::North => Box::new((x - 1..=x + 1).zip(iter::repeat(y - 1))),
                        Direction::South => Box::new((x - 1..=x + 1).zip(iter::repeat(y + 1))),
                        Direction::East => Box::new(iter::repeat(x + 1).zip(y - 1..=y + 1)),
                        Direction::West => Box::new(iter::repeat(x - 1).zip(y - 1..=y + 1)),
                    };

                    if neighbors.all(|neighbor| !self.elves.contains(&neighbor)) {
                        let proposal = match direction {
                            Direction::North => (*x, y - 1),
                            Direction::South => (*x, y + 1),
                            Direction::East => (x + 1, *y),
                            Direction::West => (x - 1, *y),
                        };

                        proposals.entry(proposal).or_default().push((*x, *y));

                        break;
                    }
                }
            }
        }

        for (proposal, elves) in proposals {
            if elves.len() == 1 {
                self.elves.remove(&elves[0]);
                self.elves.insert(proposal);
            }
        }

        self.round += 1;
    }
}

impl FromStr for Grove {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut elves = HashSet::new();

        for (y, line) in string.lines().enumerate() {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    elves.insert((x as i32, y as i32));
                }
            });
        }

        Ok(Grove { elves, round: 0 })
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ((x_min, y_min), (x_max, y_max)) = self.bounds();

        for y in y_min..=y_max {
            let mut line = String::new();

            for x in x_min..=x_max {
                line.push(if self.elves.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                });
            }

            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_GROVE: &str = indoc! {"\
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    "};

    #[test]
    fn test_open_empty_tiles() {
        let grove = Grove::from_str(TEST_GROVE).unwrap();
        assert_eq!(27, grove.empty_ground_tiles());
    }

    #[test]
    fn test_advance_round() {
        let mut grove = Grove::from_str(TEST_GROVE).unwrap();

        for round in 0..10 {
            grove.advance_round();
        }

        assert_eq!(110, grove.empty_ground_tiles());
    }
}
