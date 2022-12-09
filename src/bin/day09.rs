use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let motions: Vec<Motion> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| Motion::from_str(&line))
            .collect::<Result<_, _>>()?;

        {
            let mut rope = Rope::new(2);

            motions.iter().for_each(|motion| rope.apply(motion));

            println!(
                "Distinct positions visited by tail of 2-knot rope: {}",
                rope.distinct_tail_positions.len()
            );
        }

        {
            let mut rope = Rope::new(10);

            motions.iter().for_each(|motion| rope.apply(motion));

            println!(
                "Distinct positions visited by tail of 10-knot rope: {}",
                rope.distinct_tail_positions.len()
            );
        }

        Ok(())
    } else {
        Err("Usage: day09 INPUT_FILE_PATH".into())
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Unrecognized direction".into()),
        }
    }
}

struct Motion {
    direction: Direction,
    magnitude: i32,
}

struct Rope {
    positions: Vec<(i32, i32)>,
    distinct_tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        Rope {
            positions: vec![(0, 0); knots],
            distinct_tail_positions: HashSet::from([(0, 0)]),
        }
    }

    fn apply(&mut self, motion: &Motion) {
        let (delta_x, delta_y) = match motion.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..motion.magnitude {
            self.positions[0].0 += delta_x;
            self.positions[0].1 += delta_y;

            for knot in 1..self.positions.len() {
                // Only move a knot if it's not adjacent the knot in front of it
                if (self.positions[knot].0 - self.positions[knot - 1].0).abs() > 1
                    || (self.positions[knot].1 - self.positions[knot - 1].1).abs() > 1
                {
                    self.positions[knot].0 +=
                        match self.positions[knot - 1].0.cmp(&self.positions[knot].0) {
                            Ordering::Greater => 1,
                            Ordering::Equal => 0,
                            Ordering::Less => -1,
                        };

                    self.positions[knot].1 +=
                        match self.positions[knot - 1].1.cmp(&self.positions[knot].1) {
                            Ordering::Greater => 1,
                            Ordering::Equal => 0,
                            Ordering::Less => -1,
                        };
                } else {
                    // No knots farther down the chain will move if this knot didn't move
                    break;
                }
            }

            self.distinct_tail_positions
                .insert(self.positions[self.positions.len() - 1]);
        }
    }
}

impl FromStr for Motion {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let [direction, magnitude] = string.split(' ').collect::<Vec<&str>>().as_slice() {
            Ok(Motion {
                direction: Direction::from_str(direction)?,
                magnitude: magnitude.parse()?,
            })
        } else {
            Err("Could not parse motion".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MOTIONS: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    const LONGER_TEST_MOTIONS: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn test_distinct_tail_positions() {
        {
            let motions: Vec<Motion> = TEST_MOTIONS
                .lines()
                .map(Motion::from_str)
                .collect::<Result<_, _>>()
                .unwrap();

            {
                let mut rope = Rope::new(2);

                motions.iter().for_each(|motion| rope.apply(motion));

                assert_eq!(13, rope.distinct_tail_positions.len());
            }

            {
                let mut rope = Rope::new(10);

                motions.iter().for_each(|motion| rope.apply(motion));

                assert_eq!(1, rope.distinct_tail_positions.len());
            }
        }

        let motions: Vec<Motion> = LONGER_TEST_MOTIONS
            .lines()
            .map(Motion::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        let mut rope = Rope::new(10);

        motions.iter().for_each(|motion| rope.apply(motion));

        assert_eq!(36, rope.distinct_tail_positions.len());
    }
}
