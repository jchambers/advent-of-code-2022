use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let rucksacks: Vec<Rucksack> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.map(|items| Rucksack { items }).ok())
            .collect();

        {
            let misplaced_item_priority_sum: u32 = rucksacks.iter()
                .filter_map(|rucksack| rucksack.find_misplaced_item())
                .map(Rucksack::priority)
                .sum();

            println!("Priority sum for misplaced items: {}", misplaced_item_priority_sum);
        }

        {
            let common_item_priority_sum: u32 = rucksacks.chunks_exact(3)
                .filter_map(|chunk| Rucksack::find_common_item(&chunk[0], &chunk[1], &chunk[2]))
                .map(Rucksack::priority)
                .sum();

            println!("Priority sum for common items: {}", common_item_priority_sum);
        }

        Ok(())
    } else {
        Err("Usage: day03 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug)]
struct Rucksack {
    items: String,
}

impl Rucksack {
    fn find_misplaced_item(&self) -> Option<char> {
        let items: Vec<char> = self.items.chars().collect();

        let first_compartment = &items[0..items.len() / 2];
        let second_compartment = &items[(items.len() / 2)..];

        first_compartment.iter()
            .find(|candidate| second_compartment.contains(candidate))
            .copied()
    }

    fn priority(item: char) -> u32 {
        match item {
            'a'..='z' => item as u32 - 'a' as u32 + 1,
            'A'..='Z' => item as u32 - 'A' as u32 + 27,
            _ => panic!()
        }
    }

    fn find_common_item(a: &Rucksack, b: &Rucksack, c: &Rucksack) -> Option<char> {
        a.items.chars()
            .find(|&candidate| b.items.contains(candidate) && c.items.contains(candidate))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_misplaced_item() {
        assert_eq!(Some('p'), Rucksack { items: String::from("vJrwpWtwJgWrhcsFMMfFFhFp") }.find_misplaced_item());
        assert_eq!(Some('L'), Rucksack { items: String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL") }.find_misplaced_item());
        assert_eq!(Some('P'), Rucksack { items: String::from("PmmdzqPrVvPwwTWBwg") }.find_misplaced_item());
        assert_eq!(Some('v'), Rucksack { items: String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn") }.find_misplaced_item());
        assert_eq!(Some('t'), Rucksack { items: String::from("ttgJtRGJQctTZtZT") }.find_misplaced_item());
        assert_eq!(Some('s'), Rucksack { items: String::from("CrZsJsPPZsGzwwsLwLmpwMDw") }.find_misplaced_item());
    }

    #[test]
    fn test_priority() {
        assert_eq!(16, Rucksack::priority('p'));
        assert_eq!(38, Rucksack::priority('L'));
        assert_eq!(42, Rucksack::priority('P'));
        assert_eq!(22, Rucksack::priority('v'));
        assert_eq!(20, Rucksack::priority('t'));
        assert_eq!(19, Rucksack::priority('s'));
    }

    #[test]
    fn test_find_common_item() {
        assert_eq!(Some('r'), Rucksack::find_common_item(
            &Rucksack { items: String::from("vJrwpWtwJgWrhcsFMMfFFhFp") },
            &Rucksack { items: String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL") },
            &Rucksack { items: String::from("PmmdzqPrVvPwwTWBwg") },
        ));

        assert_eq!(Some('Z'), Rucksack::find_common_item(
            &Rucksack { items: String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn") },
            &Rucksack { items: String::from("ttgJtRGJQctTZtZT") },
            &Rucksack { items: String::from("CrZsJsPPZsGzwwsLwLmpwMDw") },
        ));
    }
}
