extern crate core;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let contained_pairs = BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok())
                .filter_map(|line| parse_assignment_pair(&line).ok())
                .filter(|(a, b)| a.contains(b) || b.contains(a))
                .count();

            println!("Pairs in which one assignment fully contains the other: {}", contained_pairs);
        }

        {
            let overlapping_pairs = BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok())
                .filter_map(|line| parse_assignment_pair(&line).ok())
                .filter(|(a, b)| a.overlaps(b))
                .count();

            println!("Pairs in which one assignment overlaps the other: {}", overlapping_pairs);
        }

        Ok(())
    } else {
        Err("Usage: day04 INPUT_FILE_PATH".into())
    }
}

fn parse_assignment_pair(string: &str) -> Result<(SectionAssignment, SectionAssignment), Box<dyn Error>> {
    if let [a, b] = string.split(',').collect::<Vec<&str>>().as_slice() {
        Ok((SectionAssignment::from_str(a)?, SectionAssignment::from_str(b)?))
    } else {
        Err("Could not parse assignment pair".into())
    }
}

struct SectionAssignment {
    start: u32,
    end: u32,
}

impl FromStr for SectionAssignment {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let [start, end] = string.split('-').collect::<Vec<&str>>().as_slice() {
            Ok(SectionAssignment {
                start: start.parse()?,
                end: end.parse()?,
            })
        } else {
            Err("Could not parse section assignment".into())
        }
    }
}

impl SectionAssignment {
    fn contains(&self, other: &SectionAssignment) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &SectionAssignment) -> bool {
        other.end >= self.start && other.start <= self.end
    }
}

#[cfg(test)]
mod test {
    use crate::SectionAssignment;

    #[test]
    fn test_section_assignment_contains() {
        assert!(SectionAssignment { start: 2, end: 8 }.contains(&SectionAssignment { start: 3, end: 7 }));
        assert!(SectionAssignment { start: 4, end: 6 }.contains(&SectionAssignment { start: 6, end: 6 }));
        assert!(!SectionAssignment { start: 3, end: 7 }.contains(&SectionAssignment { start: 2, end: 8 }))
    }

    #[test]
    fn test_section_assignment_overlaps() {
        assert!(SectionAssignment { start: 2, end: 8 }.overlaps(&SectionAssignment { start: 3, end: 7 }));
        assert!(SectionAssignment { start: 4, end: 6 }.overlaps(&SectionAssignment { start: 6, end: 6 }));
        assert!(SectionAssignment { start: 3, end: 7 }.overlaps(&SectionAssignment { start: 2, end: 8 }));

        assert!(SectionAssignment { start: 5, end: 7 }.overlaps(&SectionAssignment { start: 7, end: 9 }));
        assert!(SectionAssignment { start: 6, end: 6 }.overlaps(&SectionAssignment { start: 4, end: 6 }));
        assert!(SectionAssignment { start: 2, end: 6 }.overlaps(&SectionAssignment { start: 4, end: 8 }));

        assert!(!SectionAssignment { start: 2, end: 4 }.overlaps(&SectionAssignment { start: 6, end: 8 }));
        assert!(!SectionAssignment { start: 2, end: 3 }.overlaps(&SectionAssignment { start: 4, end: 5 }));
    }
}
