use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut elves = Elf::try_from_calorie_list(
            BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok()),
        )?;

        elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
        let elves = elves;

        println!("Max calories carried by single elf: {}", top_calorie_total(&elves, 1));
        println!("Calorie total for top three elves: {:?}", top_calorie_total(&elves, 3));

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

fn top_calorie_total(elves: &[Elf], count: usize) -> u32 {
    // Sadly, `is_sorted` is still a nightly/experimental feature
    // assert!(elves.is_sorted());

    elves.iter()
        .take(count)
        .map(|elf| elf.total_calories())
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn try_from_calorie_list(lines: impl Iterator<Item = String>) -> Result<Vec<Elf>, Box<dyn Error>> {
        let mut elves = vec![];

        for (empty, group) in &lines.group_by(|line| line.is_empty()) {
            if !empty {
                elves.push(Elf { calories: group.map(|line| line.parse::<u32>())
                    .collect::<Result<Vec<u32>, _>>()?
                });
            }
        }

        Ok(elves)
    }

    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use indoc::indoc;

    const TEST_INPUT: &'static str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn test_try_from_calorie_list() {
        let expected = vec![
            Elf { calories: vec![1000, 2000, 3000] },
            Elf { calories: vec![4000] },
            Elf { calories: vec![5000, 6000] },
            Elf { calories: vec![7000, 8000, 9000] },
            Elf { calories: vec![10000] },
        ];

        assert_eq!(
            expected,
            Elf::try_from_calorie_list(TEST_INPUT.lines().map(|line| String::from(line))).unwrap()
        );
    }
}
