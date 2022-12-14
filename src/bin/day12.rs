use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let height_map = HeightMap::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Shortest path to exit from designated start: {}",
            height_map.shortest_path(height_map.start).unwrap()
        );

        println!(
            "Shortest path to exit from any starting point: {}",
            height_map
                .possible_starting_positions()
                .iter()
                .filter_map(|start| height_map.shortest_path(*start))
                .min()
                .unwrap()
        );

        Ok(())
    } else {
        Err("Usage: day12 INPUT_FILE_PATH".into())
    }
}

struct HeightMap {
    heights: Vec<u8>,
    width: usize,

    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for HeightMap {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let width = string
            .lines()
            .next()
            .ok_or("Hieght map string must be at least one line")?
            .len();

        let mut start_index = 0;
        let mut end_index = 0;

        let heights: Vec<u8> = string
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .enumerate()
            .map(|(i, c)| match c {
                'S' => {
                    start_index = i;
                    0
                }
                'E' => {
                    end_index = i;
                    25
                }
                _ => c as u8 - b'a',
            })
            .collect();

        if heights.len() % width == 0 {
            Ok(HeightMap {
                heights,
                width,

                start: (start_index % width, start_index / width),
                end: (end_index % width, end_index / width),
            })
        } else {
            Err("Height map does not have consistent width".into())
        }
    }
}

impl HeightMap {
    fn shortest_path(&self, start: (usize, usize)) -> Option<usize> {
        let mut exploration_queue = VecDeque::new();
        let mut explored_positions = HashSet::new();
        let mut entry_points = HashMap::new();

        explored_positions.insert(start);
        exploration_queue.push_back(start);

        loop {
            if exploration_queue.is_empty() {
                return None;
            }

            let position = exploration_queue.pop_front().unwrap();

            if position == self.end {
                break;
            } else {
                for neighbor in self.neighbors(position.0, position.1) {
                    if self.height(neighbor) <= self.height(position) + 1
                        && !explored_positions.contains(&neighbor)
                    {
                        explored_positions.insert(neighbor);
                        exploration_queue.push_back(neighbor);

                        entry_points.insert(neighbor, position);
                    }
                }
            }
        }

        // At this point, we've mapped out the shortest path; now we just need to fish it out of
        // the entry point map.
        let mut steps = 0;
        let mut position = self.end;

        while position != start {
            position = *entry_points.get(&position).unwrap();
            steps += 1;
        }

        Some(steps)
    }

    fn height(&self, position: (usize, usize)) -> u8 {
        self.heights[position.0 + (self.width * position.1)]
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if y < (self.heights.len() / self.width) - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn possible_starting_positions(&self) -> Vec<(usize, usize)> {
        self.heights
            .iter()
            .enumerate()
            .filter(|(_, height)| **height == 0)
            .map(|(i, _)| (i % self.width, i / self.width))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MAP: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn test_shortest_path() {
        let height_map = HeightMap::from_str(TEST_MAP).unwrap();
        assert_eq!(31, height_map.shortest_path(height_map.start).unwrap());

        assert_eq!(
            29,
            height_map
                .possible_starting_positions()
                .iter()
                .filter_map(|start| height_map.shortest_path(*start))
                .min()
                .unwrap()
        );
    }
}
