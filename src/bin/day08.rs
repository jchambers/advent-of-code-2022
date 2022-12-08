use std::error::Error;
use std::{fs, iter};
use std::str::FromStr;
use itertools::iproduct;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let forest = Forest::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Visible trees: {}", forest.visible_trees());
        println!("Max scenic score: {}", forest.max_scenic_score());

        Ok(())
    } else {
        Err("Usage: day08 INPUT_FILE_PATH".into())
    }
}

struct Forest {
    trees: Vec<u8>,
    width: usize,
}

impl FromStr for Forest {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let width = string.lines()
            .next()
            .expect("Forest string must have at least one line")
            .len();

        let trees: Vec<u8> = string.chars()
            .filter(|c| c.is_numeric())
            .map(|c| c as u8 - b'0')
            .collect();

        if trees.len() % width != 0 || trees.len() / width != width {
            Err("Forest is not square".into())
        } else {
            Ok(Forest { trees, width })
        }
    }
}

impl Forest {
    fn index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    fn tree_height(&self, x: usize, y: usize) -> u8 {
        self.trees[self.index(x, y)]
    }

    fn visible_trees(&self) -> usize {
        let mut visible_trees = vec![false; self.trees.len()];

        iproduct!(0..self.width, 0..self.width)
            .filter(|(x, y)| self.visible(*x, *y))
            .for_each(|(x, y)| visible_trees[self.index(x, y)] = true);

        visible_trees.iter()
            .filter(|&&visible| visible)
            .count()
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        let mut rays: [Box<dyn Iterator<Item = (usize, usize)>>; 4] = [
            // From the left
            Box::new((0..x).zip(iter::repeat(y))),

            // From the right
            Box::new((x + 1..self.width).zip(iter::repeat(y))),

            // From above
            Box::new(iter::repeat(x).zip(0..y)),

            // From below
            Box::new(iter::repeat(x).zip(y + 1..self.width)),
        ];

        let self_height = self.tree_height(x, y);

        rays.iter_mut()
            .any(|ray| ray.all(|(a, b)| self.tree_height(a, b) < self_height))
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let mut rays: [Box<dyn Iterator<Item = (usize, usize)>>; 4] = [
            // To the left
            Box::new((0..x).rev().zip(iter::repeat(y))),

            // To the right
            Box::new((x + 1..self.width).zip(iter::repeat(y))),

            // Above
            Box::new(iter::repeat(x).zip((0..y).rev())),

            // Below
            Box::new(iter::repeat(x).zip(y + 1..self.width)),
        ];

        rays.iter_mut()
            .map(|ray| ray
                .map(|(a, b)| self.tree_height(a, b))
                .scan(false, |blocked: &mut bool, height: u8|
                    if *blocked {
                        None
                    } else {
                        if height >= self.tree_height(x, y) {
                            // This is a little sneaky; if we get blocked by a tree, we can still
                            // see the blocking tree and want to count it. It's the NEXT one (i.e.
                            // the tree BEHIND the tall tree) that we can't see.
                            *blocked = true;
                        }

                        Some(height)
                    })
                .count())
            .product()
    }

    fn max_scenic_score(&self) -> usize {
        iproduct!(0..self.width, 0..self.width)
            .map(|(x, y)| self.scenic_score(x, y))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_FOREST: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_visible_trees() {
        let forest = Forest::from_str(TEST_FOREST).unwrap();

        assert_eq!(21, forest.visible_trees());
    }

    #[test]
    fn test_max_scenic_score() {
        let forest = Forest::from_str(TEST_FOREST).unwrap();

        assert_eq!(8, forest.max_scenic_score());
    }
}
