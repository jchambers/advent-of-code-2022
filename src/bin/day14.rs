use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let mut cave = SandCave::from_str(fs::read_to_string(path)?.as_str())?;
        cave.settle_sand();

        println!(
            "Grains of sand at rest: {}",
            cave.cells
                .iter()
                .filter(|cell| matches!(cell, Cell::Sand))
                .count()
        );

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

struct SandCave {
    cells: Vec<Cell>,

    x_bounds: (usize, usize),
    y_max: usize,
}

impl FromStr for SandCave {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let paths: Vec<RockPath> = string
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(RockPath::from_str)
            .collect::<Result<_, _>>()?;

        let bounds = paths
            .iter()
            .flat_map(|path| &path.vertices)
            .fold(((usize::MAX, usize::MIN), usize::MIN), |bounds, (x, y)| {
                ((bounds.0 .0.min(*x), bounds.0 .1.max(*x)), bounds.1.max(*y))
            });

        // +1 for the inclusive range, then +2 for one cell of padding for sand to fall at either
        // end
        let width = bounds.0 .0.abs_diff(bounds.0 .1) + 3;
        let height = bounds.1 + 1;

        let mut cave = SandCave {
            cells: vec![Cell::Empty; width * height],

            // Add padding for sand to fall at either end
            x_bounds: (bounds.0 .0 - 1, bounds.0 .1 + 1),
            y_max: bounds.1,
        };

        paths.iter().try_for_each(|path| cave.add_rock_path(path))?;

        Ok(cave)
    }
}

impl Display for SandCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.x_bounds.1 - self.x_bounds.0 + 1;

        self.cells
            .chunks_exact(width)
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Cell::Empty => '.',
                        Cell::Rock => '#',
                        Cell::Sand => 'o',
                    })
                    .collect::<String>()
            })
            .try_for_each(|line| writeln!(f, "{}", line))?;

        Ok(())
    }
}

impl SandCave {
    fn index(&self, x: usize, y: usize) -> usize {
        (x - self.x_bounds.0) + ((self.x_bounds.1 - self.x_bounds.0 + 1) * y)
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
                    let index = self.index(x, y);
                    self.cells[index] = Cell::Rock;
                }
            }
        }

        Ok(())
    }

    fn settle_sand(&mut self) {
        while self.add_grain_of_sand().is_ok() {}
    }

    fn add_grain_of_sand(&mut self) -> Result<(usize, usize), ()> {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y == self.y_max {
                // We've fallen off the bottom
                break Err(());
            }

            if let Some(updated_x) =
                match &self.cells[self.index(x - 1, y + 1)..=self.index(x + 1, y + 1)] {
                    [_, Cell::Empty, _] => Some(x),
                    [Cell::Empty, _, _] => Some(x - 1),
                    [_, _, Cell::Empty] => Some(x + 1),
                    _ => None,
                }
            {
                x = updated_x;
                y += 1;
            } else {
                // The grain of sand has nowhere left to go and is settled
                let index = self.index(x, y);
                self.cells[index] = Cell::Sand;

                break Ok((x, y));
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
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

    #[test]
    fn test_add_grain_of_sand() {
        let mut cave = SandCave::from_str(TEST_PATHS).unwrap();

        assert!(matches!(cave.cells[cave.index(500, 8)], Cell::Empty));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cells[cave.index(500, 8)], Cell::Sand));

        assert!(matches!(cave.cells[cave.index(499, 8)], Cell::Empty));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cells[cave.index(499, 8)], Cell::Sand));

        assert!(matches!(cave.cells[cave.index(501, 8)], Cell::Empty));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cells[cave.index(501, 8)], Cell::Sand));

        assert!(matches!(cave.cells[cave.index(500, 7)], Cell::Empty));
        cave.add_grain_of_sand().unwrap();
        assert!(matches!(cave.cells[cave.index(500, 7)], Cell::Sand));
    }

    #[test]
    fn test_settle_sand() {
        let mut cave = SandCave::from_str(TEST_PATHS).unwrap();
        cave.settle_sand();

        assert_eq!(
            24,
            cave.cells
                .iter()
                .filter(|cell| matches!(cell, Cell::Sand))
                .count()
        );
    }
}
