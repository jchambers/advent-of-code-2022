use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let mut cave = SandCave::from_str(fs::read_to_string(path)?.as_str())?;
            cave.settle_sand();

            println!(
                "Grains of sand at rest in bottomless cave: {}",
                cave.cells
                    .values()
                    .filter(|cell| matches!(cell, Cell::Sand))
                    .count()
            );
        }

        {
            let mut cave = SandCave::from_str(fs::read_to_string(path)?.as_str())?;
            cave.has_floor = true;

            cave.settle_sand();

            println!(
                "Grains of sand at rest in cave with floor: {}",
                cave.cells
                    .values()
                    .filter(|cell| matches!(cell, Cell::Sand))
                    .count()
            );
        }

        Ok(())
    } else {
        Err("Usage: day14 INPUT_FILE_PATH".into())
    }
}

struct SandCave {
    cells: HashMap<(usize, usize), Cell>,
    y_max: usize,
    has_floor: bool,
}

impl FromStr for SandCave {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let paths: Vec<RockPath> = string
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(RockPath::from_str)
            .collect::<Result<_, _>>()?;

        let y_max = paths
            .iter()
            .flat_map(|path| &path.vertices)
            .map(|vertex| vertex.1)
            .max()
            .expect("Rock paths must not be empty");

        let mut cave = SandCave {
            cells: HashMap::new(),
            y_max,
            has_floor: false,
        };

        paths.iter().try_for_each(|path| cave.add_rock_path(path))?;

        Ok(cave)
    }
}

impl Display for SandCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x_min, x_max) = self
            .cells
            .keys()
            .map(|(x, _)| *x)
            .fold((usize::MAX, usize::MIN), |bounds, x| {
                (bounds.0.min(x), bounds.1.max(x))
            });

        let y_range = if self.has_floor {
            0..=self.y_max + 2
        } else {
            0..=self.y_max
        };

        for y in y_range {
            let row: String = (x_min..=x_max)
                .map(|x| {
                    if x == 500 && y == 0 {
                        '+'
                    } else {
                        match self.cell_at(x, y) {
                            Some(Cell::Rock) => '#',
                            Some(Cell::Sand) => 'o',
                            _ => '.',
                        }
                    }
                })
                .collect();

            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

impl SandCave {
    fn cell_at(&self, x: usize, y: usize) -> Option<Cell> {
        if self.has_floor && y == self.y_max + 2 {
            Some(Cell::Rock)
        } else {
            self.cells.get(&(x, y)).copied()
        }
    }

    fn add_rock_path(&mut self, path: &RockPath) -> Result<(), Box<dyn Error>> {
        for i in 0..path.vertices.len() - 1 {
            if path.vertices[i].0 != path.vertices[i + 1].0
                && path.vertices[i].1 != path.vertices[i + 1].1
            {
                return Err("Path is neither horizontal nor vertical".into());
            }

            for x in path.vertices[i].0.min(path.vertices[i + 1].0)
                ..=path.vertices[i].0.max(path.vertices[i + 1].0)
            {
                for y in path.vertices[i].1.min(path.vertices[i + 1].1)
                    ..=path.vertices[i].1.max(path.vertices[i + 1].1)
                {
                    self.cells.insert((x, y), Cell::Rock);
                }
            }
        }

        Ok(())
    }

    fn settle_sand(&mut self) {
        while self.add_grain_of_sand().is_ok() {
            // We've blocked the sand spout
            if matches!(self.cell_at(500, 0), Some(Cell::Sand)) {
                break;
            }
        }
    }

    fn add_grain_of_sand(&mut self) -> Result<(usize, usize), ()> {
        let mut x = 500;
        let mut y = 0;

        loop {
            if !self.has_floor && y >= self.y_max {
                // We've fallen off the bottom
                break Err(());
            }

            let candidates = [
                self.cell_at(x - 1, y + 1),
                self.cell_at(x, y + 1),
                self.cell_at(x + 1, y + 1),
            ];

            if let Some(updated_x) = match &candidates {
                [_, None, _] => Some(x),
                [None, _, _] => Some(x - 1),
                [_, _, None] => Some(x + 1),
                _ => None,
            } {
                x = updated_x;
                y += 1;
            } else {
                // The grain of sand has nowhere left to go and is settled
                self.cells.insert((x, y), Cell::Sand);
                break Ok((x, y));
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Rock,
    Sand,
}

struct RockPath {
    vertices: Vec<(usize, usize)>,
}

impl FromStr for RockPath {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let vertices: Vec<(usize, usize)> = string
            .split(" -> ")
            .map(|pair| {
                if let [x, y] = pair.split(',').collect::<Vec<&str>>().as_slice() {
                    Ok((x.parse()?, y.parse()?))
                } else {
                    Err("Could not split pair".into())
                }
            })
            .collect::<Result<_, Box<dyn Error>>>()?;

        Ok(RockPath { vertices })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_PATHS: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    fn _display_sand_cave() {
        println!("{}", SandCave::from_str(TEST_PATHS).unwrap())
    }

    fn _display_sand_cave_with_floor() {
        let mut cave = SandCave::from_str(TEST_PATHS).unwrap();
        cave.has_floor = true;

        println!("{}", cave);
    }

    #[test]
    fn test_add_grain_of_sand() {
        let mut cave = SandCave::from_str(TEST_PATHS).unwrap();

        assert!(matches!(cave.cell_at(500, 8), None));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cell_at(500, 8), Some(Cell::Sand)));

        assert!(matches!(cave.cell_at(499, 8), None));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cell_at(499, 8), Some(Cell::Sand)));

        assert!(matches!(cave.cell_at(501, 8), None));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cell_at(501, 8), Some(Cell::Sand)));

        assert!(matches!(cave.cell_at(500, 7), None));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cell_at(500, 7), Some(Cell::Sand)));
    }

    #[test]
    fn test_settle_sand() {
        let mut cave = SandCave::from_str(TEST_PATHS).unwrap();
        cave.settle_sand();

        assert_eq!(
            24,
            cave.cells
                .values()
                .filter(|cell| matches!(cell, Cell::Sand))
                .count()
        );
    }

    #[test]
    fn test_settle_sand_with_floor() {
        let mut cave = SandCave::from_str(TEST_PATHS).unwrap();
        cave.has_floor = true;

        cave.settle_sand();

        assert_eq!(
            93,
            cave.cells
                .values()
                .filter(|cell| matches!(cell, Cell::Sand))
                .count()
        );
    }
}
