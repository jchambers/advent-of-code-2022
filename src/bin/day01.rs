use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let elves = Elf::try_from_calorie_list(
            BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok()),
        )?;

        println!(
            "Max calories carried by single elf: {}",
            elves.iter().map(|elf| elf.total_calories()).max().unwrap()
        );

        let mut calorie_totals: Vec<u32> = elves.iter().map(|elf| elf.total_calories()).collect();

        calorie_totals.sort();
        let top_three_total: u32 = calorie_totals.iter().rev().take(3).sum();

        println!("Calorie total for top three elves: {:?}", top_three_total);

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn try_from_calorie_list(lines: impl Iterator<Item = String>) -> Result<Vec<Elf>, Box<dyn Error>> {
        let mut elves = vec![];
        let mut calorie_buffer = vec![];

        for line in lines {
            if line.is_empty() {
                elves.push(Elf { calories: calorie_buffer.clone() });
                calorie_buffer.clear();
            } else {
                calorie_buffer.push(line.parse::<u32>()?);
            }
        }

        if !calorie_buffer.is_empty() {
            elves.push(Elf { calories: calorie_buffer.clone(), });
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
