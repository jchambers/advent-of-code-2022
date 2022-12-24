use std::collections::{HashSet, LinkedList};
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let valley = BlizzardValley::from_str(fs::read_to_string(path)?.as_str())?;
            println!("Time to reach exit: {}", valley.fastest_time_to_exit());
        }

        {
            let valley = BlizzardValley::from_str(fs::read_to_string(path)?.as_str())?;

            println!(
                "Time to reach exit, then entrance, then exit: {}",
                valley.fastest_time_to_exit_with_return_to_start()
            );
        }

        Ok(())
    } else {
        Err("Usage: day24 INPUT_FILE_PATH".into())
    }
}

struct BlizzardValley {
    width: usize,
    height: usize,

    elapsed_time: u32,

    left_blizzards_by_row: Vec<LinkedList<bool>>,
    right_blizzards_by_row: Vec<LinkedList<bool>>,
    up_blizzards_by_col: Vec<LinkedList<bool>>,
    down_blizzards_by_col: Vec<LinkedList<bool>>,
}

impl BlizzardValley {
    fn fastest_time_to_exit(mut self) -> u32 {
        let start = (0, 0);
        let end = (self.width - 1, self.height - 1);

        self.navigate(start, end);

        // We need one additional unit of time to step through the exit
        self.elapsed_time + 1
    }

    fn fastest_time_to_exit_with_return_to_start(mut self) -> u32 {
        let start = (0, 0);
        let end = (self.width - 1, self.height - 1);

        // Find a path from the start to the exit
        self.navigate(start, end);

        // Advance one unit of time while we step through the exit
        self.advance_time();

        // Find a path back from the exit to the entrance
        self.navigate(end, start);

        // Step back out through the entrance
        self.advance_time();

        // Find a path BACK to the exit
        self.navigate(start, end);

        // …and we need one additional unit of time to step through the exit
        self.elapsed_time + 1
    }

    fn navigate(&mut self, start: (usize, usize), end: (usize, usize)) {
        let mut reachable_positions = HashSet::new();

        loop {
            if reachable_positions.contains(&end) {
                break;
            }

            self.advance_time();
            let empty_positions = self.empty_positions();

            let mut next_reachable_positions = HashSet::new();

            if empty_positions.contains(&start) {
                next_reachable_positions.insert(start);
            }

            for (x, y) in reachable_positions {
                next_reachable_positions.extend(
                    self.neighbors(x, y)
                        .iter()
                        .filter(|position| empty_positions.contains(position)),
                );
            }

            reachable_positions = next_reachable_positions;
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        // Include the current position as a "neighbor" to explore
        let mut neighbors = vec![(x, y)];

        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn advance_time(&mut self) {
        self.left_blizzards_by_row.iter_mut().for_each(|row| {
            let cell = row.pop_front().unwrap();
            row.push_back(cell);
        });

        self.right_blizzards_by_row.iter_mut().for_each(|row| {
            let cell = row.pop_back().unwrap();
            row.push_front(cell);
        });

        self.up_blizzards_by_col.iter_mut().for_each(|col| {
            let cell = col.pop_front().unwrap();
            col.push_back(cell);
        });

        self.down_blizzards_by_col.iter_mut().for_each(|col| {
            let cell = col.pop_back().unwrap();
            col.push_front(cell);
        });

        self.elapsed_time += 1;
    }

    fn empty_positions(&self) -> HashSet<(usize, usize)> {
        let mut spaces_with_blizzards = vec![false; self.width * self.height];

        for y in 0..self.height {
            self.left_blizzards_by_row[y]
                .iter()
                .enumerate()
                .filter(|(_, occupied)| **occupied)
                .for_each(|(x, _)| {
                    spaces_with_blizzards[(y * self.width) + x] = true;
                });

            self.right_blizzards_by_row[y]
                .iter()
                .enumerate()
                .filter(|(_, occupied)| **occupied)
                .for_each(|(x, _)| {
                    spaces_with_blizzards[(y * self.width) + x] = true;
                });
        }

        for x in 0..self.width {
            self.up_blizzards_by_col[x]
                .iter()
                .enumerate()
                .filter(|(_, occupied)| **occupied)
                .for_each(|(y, _)| {
                    spaces_with_blizzards[(y * self.width) + x] = true;
                });

            self.down_blizzards_by_col[x]
                .iter()
                .enumerate()
                .filter(|(_, occupied)| **occupied)
                .for_each(|(y, _)| {
                    spaces_with_blizzards[(y * self.width) + x] = true;
                });
        }

        spaces_with_blizzards
            .iter()
            .enumerate()
            .filter(|(_, has_blizzard)| !**has_blizzard)
            .map(|(i, _)| (i % self.width, i / self.width))
            .collect()
    }
}

impl FromStr for BlizzardValley {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let width = string.lines().next().ok_or("Map must not be blank")?.len() - 2;
        let height = string.lines().filter(|line| !line.is_empty()).count() - 2;

        let mut y = 0;

        let mut left_blizzards_by_row: Vec<LinkedList<bool>> = vec![LinkedList::new(); height];
        let mut right_blizzards_by_row: Vec<LinkedList<bool>> = vec![LinkedList::new(); height];
        let mut up_blizzards_by_col: Vec<LinkedList<bool>> = vec![LinkedList::new(); width];
        let mut down_blizzards_by_col: Vec<LinkedList<bool>> = vec![LinkedList::new(); width];

        string
            .lines()
            .skip_while(|line| line.starts_with("#."))
            .take_while(|line| !line.starts_with("##"))
            .for_each(|line| {
                line.chars()
                    .filter(|&c| c != '#')
                    .enumerate()
                    .for_each(|(x, c)| {
                        left_blizzards_by_row[y].push_back(c == '<');
                        right_blizzards_by_row[y].push_back(c == '>');
                        up_blizzards_by_col[x].push_back(c == '^');
                        down_blizzards_by_col[x].push_back(c == 'v');
                    });

                y += 1;
            });

        Ok(Self {
            width,
            height,

            elapsed_time: 0,

            left_blizzards_by_row,
            right_blizzards_by_row,
            up_blizzards_by_col,
            down_blizzards_by_col,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_VALLEY: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn test_advance_time() {
        let mut valley = BlizzardValley::from_str(TEST_VALLEY).unwrap();

        {
            let expected_empty_positions = HashSet::from([(2, 0), (0, 1), (2, 1), (3, 1), (2, 2)]);
            assert_eq!(expected_empty_positions, valley.empty_positions());
        }

        valley.advance_time();

        {
            let expected_empty_positions = HashSet::from([
                (0, 0),
                (3, 0),
                (5, 0),
                (1, 1),
                (2, 1),
                (5, 1),
                (2, 2),
                (5, 2),
                (2, 3),
                (3, 3),
            ]);

            assert_eq!(expected_empty_positions, valley.empty_positions());
        }
    }

    #[test]
    fn test_fastest_time_to_exit() {
        let valley = BlizzardValley::from_str(TEST_VALLEY).unwrap();
        assert_eq!(18, valley.fastest_time_to_exit());
    }

    #[test]
    fn test_fastest_time_to_exit_with_return_to_start() {
        let valley = BlizzardValley::from_str(TEST_VALLEY).unwrap();
        assert_eq!(54, valley.fastest_time_to_exit_with_return_to_start());
    }
}
