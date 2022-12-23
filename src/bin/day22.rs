use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let map = MonkeyMap::from_str(fs::read_to_string(path)?.as_str())?;
        println!("Password from map: {}", map.password());

        Ok(())
    } else {
        Err("Usage: day22 INPUT_FILE_PATH".into())
    }
}

struct MonkeyMap {
    tiles: Vec<MapTile>,
    width: usize,

    instructions: Vec<Instruction>,
}

impl MonkeyMap {
    fn index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    fn password(&self) -> u32 {
        let mut heading = Heading::Right;
        let mut y = 0;
        let mut x = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, tile)| !matches!(tile, MapTile::Void))
            .unwrap()
            .0;

        for instruction in &self.instructions {
            match instruction {
                Instruction::TurnLeft => {
                    heading = match heading {
                        Heading::Up => Heading::Left,
                        Heading::Down => Heading::Right,
                        Heading::Left => Heading::Down,
                        Heading::Right => Heading::Up,
                    }
                }

                Instruction::TurnRight => {
                    heading = match heading {
                        Heading::Up => Heading::Right,
                        Heading::Down => Heading::Left,
                        Heading::Left => Heading::Up,
                        Heading::Right => Heading::Down,
                    }
                }

                Instruction::Advance(distance) => {
                    for _ in 0..*distance {
                        let (next_x, next_y) = self.next_tile(x, y, &heading);

                        match self.tiles[self.index(next_x, next_y)] {
                            MapTile::Open => {
                                x = next_x;
                                y = next_y;
                            }
                            MapTile::Wall => break,
                            MapTile::Void => unreachable!(),
                        }
                    }
                }
            }
        }

        let facing_score = match heading {
            Heading::Up => 3,
            Heading::Down => 1,
            Heading::Left => 2,
            Heading::Right => 0,
        };

        ((1000 * (y + 1)) + (4 * (x + 1)) + facing_score) as u32
    }

    fn next_tile(&self, x: usize, y: usize, heading: &Heading) -> (usize, usize) {
        match heading {
            Heading::Up => {
                let mut y = y;

                loop {
                    y = if y == 0 {
                        (self.tiles.len() / self.width) - 1
                    } else {
                        y - 1
                    };

                    if !matches!(self.tiles[self.index(x, y)], MapTile::Void) {
                        break (x, y);
                    }
                }
            }

            Heading::Down => {
                let mut y = y;

                loop {
                    y = (y + 1) % (self.tiles.len() / self.width);

                    if !matches!(self.tiles[self.index(x, y)], MapTile::Void) {
                        break (x, y);
                    }
                }
            }

            Heading::Left => {
                let mut x = x;

                loop {
                    x = if x == 0 { self.width - 1 } else { x - 1 };

                    if !matches!(self.tiles[self.index(x, y)], MapTile::Void) {
                        break (x, y);
                    }
                }
            }

            Heading::Right => {
                let mut x = x;

                loop {
                    x = (x + 1) % self.width;

                    if !matches!(self.tiles[self.index(x, y)], MapTile::Void) {
                        break (x, y);
                    }
                }
            }
        }
    }
}

impl FromStr for MonkeyMap {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let [map, directions] = string.split("\n\n").collect::<Vec<&str>>().as_slice() {
            let width = map
                .lines()
                .map(|line| line.len())
                .max()
                .ok_or("Map must not be blank")?;

            let tiles = {
                let height = map.lines().filter(|line| !line.is_empty()).count();
                let mut tiles = vec![MapTile::Void; width * height];

                map.lines().enumerate().for_each(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '.' || *c == '#')
                        .for_each(|(x, c)| {
                            tiles[(y * width) + x] = match c {
                                '.' => MapTile::Open,
                                '#' => MapTile::Wall,
                                _ => unreachable!(),
                            };
                        });
                });

                tiles
            };

            let instructions = {
                let mut instructions = vec![];
                let mut advance_steps = 0;

                directions.chars().for_each(|c| match c {
                    'L' => {
                        if advance_steps != 0 {
                            instructions.push(Instruction::Advance(advance_steps));
                            advance_steps = 0;
                        }

                        instructions.push(Instruction::TurnLeft);
                    }

                    'R' => {
                        if advance_steps != 0 {
                            instructions.push(Instruction::Advance(advance_steps));
                            advance_steps = 0;
                        }

                        instructions.push(Instruction::TurnRight);
                    }

                    '0'..='9' => {
                        advance_steps *= 10;
                        advance_steps += c.to_digit(10).unwrap() as usize;
                    }

                    _ => {}
                });

                if advance_steps != 0 {
                    instructions.push(Instruction::Advance(advance_steps));
                }

                instructions
            };

            Ok(MonkeyMap {
                tiles,
                width,
                instructions,
            })
        } else {
            Err("Could not parse map/directions".into())
        }
    }
}

impl Display for MonkeyMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.tiles.chunks_exact(self.width).try_for_each(|chunk| {
            let line: String = chunk
                .iter()
                .map(|tile| match tile {
                    MapTile::Open => '.',
                    MapTile::Wall => '#',
                    MapTile::Void => ' ',
                })
                .collect();

            writeln!(f, "{}", line)
        })?;

        Ok(())
    }
}

#[derive(Copy, Clone)]
enum MapTile {
    Open,
    Wall,
    Void,
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Advance(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Debug)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MAP: &str = indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5"
    };

    #[test]
    fn test_password() {
        let map = MonkeyMap::from_str(TEST_MAP).unwrap();
        assert_eq!(6032, map.password());
    }
}
