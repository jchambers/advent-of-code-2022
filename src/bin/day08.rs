use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let forest = Forest::from_str(fs::read_to_string(path)?.as_str())?;
        println!("Visible trees: {}", forest.visible_trees());

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

        for x in 0..self.width {
            for y in 0..self.width {
                if x == 0 || y == 0 || x == self.width - 1 || y == self.width - 1 {
                    visible_trees[self.index(x, y)] = true;
                    continue;
                }

                let visible_from_left = (0..x).map(|a| self.tree_height(a, y))
                    .all(|height| height < self.tree_height(x, y));

                let visible_from_right = (x + 1..self.width).map(|a| self.tree_height(a, y))
                    .all(|height| height < self.tree_height(x, y));

                let visible_from_above = (0..y).map(|b| self.tree_height(x, b))
                    .all(|height| height < self.tree_height(x, y));

                let visible_from_below = (y + 1..self.width).map(|b| self.tree_height(x, b))
                    .all(|height| height < self.tree_height(x, y));

                if visible_from_left || visible_from_right || visible_from_above || visible_from_below {
                    visible_trees[self.index(x, y)] = true;
                }
            }
        }

        visible_trees.iter()
            .filter(|&&visible| visible)
            .count()
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
}
