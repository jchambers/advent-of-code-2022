use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::RoundOutcome::{Draw, Lose, Win};
use crate::Shape::{Paper, Rock, Scissors};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let score_with_prescribed_shape: u32 = BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok())
                .filter_map(|line| Round::from_str_with_prescribed_shape(&line).ok())
                .map(|round| round.score())
                .sum();

            println!("Total score with prescribed shapes: {}", score_with_prescribed_shape);
        }

        {
            let score_with_prescribed_outcome: u32 = BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok())
                .filter_map(|line| Round::from_str_with_prescribed_outcome(&line).ok())
                .map(|round| round.score())
                .sum();

            println!("Total score with prescribed outcomes: {}", score_with_prescribed_outcome);
        }

        Ok(())
    } else {
        Err("Usage: day02 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum RoundOutcome {
    Win,
    Lose,
    Draw
}

impl RoundOutcome {
    fn score(&self) -> u32 {
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Round {
    self_shape: Shape,
    opponent_shape: Shape,
}

impl Round {
    fn from_str_with_prescribed_shape(string: &str) -> Result<Self, Box<dyn Error>> {
        if string.len() != 3 {
            return Err("Bad round string length".into());
        }

        let opponent_shape = match &string[0..1] {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err("Unrecognized opponent shape".into())
        };

        let self_shape = match &string[2..] {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => return Err("Unrecognized self shape".into())
        };

        Ok(Round { self_shape, opponent_shape })
    }

    fn from_str_with_prescribed_outcome(string: &str) -> Result<Self, Box<dyn Error>> {
        if string.len() != 3 {
            return Err("Bad round string length".into());
        }

        let opponent_shape = match &string[0..1] {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err("Unrecognized opponent shape".into())
        };

        let self_shape = match &string[2..] {
            // Lose
            "X" => match opponent_shape {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },

            // Draw
            "Y" => match opponent_shape {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            },

            // Win
            "Z" => match opponent_shape {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            _ => return Err("Unrecognized self shape".into())
        };

        Ok(Round { self_shape, opponent_shape })
    }

    fn score(&self) -> u32 {
        let outcome = match self.self_shape {
            Rock => match self.opponent_shape {
                Rock => Draw,
                Paper => Lose,
                Scissors => Win,
            }
            Paper => match self.opponent_shape {
                Rock => Win,
                Paper => Draw,
                Scissors => Lose,
            }
            Scissors => match self.opponent_shape {
                Rock => Lose,
                Paper => Win,
                Scissors => Draw,
            }
        };

        outcome.score() + self.self_shape.value()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_from_string_prescribed_shape() {
        assert_eq!(Round { opponent_shape: Rock, self_shape: Paper }, Round::from_str_with_prescribed_shape("A Y").unwrap());
        assert_eq!(Round { opponent_shape: Paper, self_shape: Rock }, Round::from_str_with_prescribed_shape("B X").unwrap());
        assert_eq!(Round { opponent_shape: Scissors, self_shape: Scissors }, Round::from_str_with_prescribed_shape("C Z").unwrap());
    }

    #[test]
    fn test_round_from_string_prescribed_outcome() {
        assert_eq!(Round { opponent_shape: Rock, self_shape: Rock }, Round::from_str_with_prescribed_outcome("A Y").unwrap());
        assert_eq!(Round { opponent_shape: Paper, self_shape: Rock }, Round::from_str_with_prescribed_outcome("B X").unwrap());
        assert_eq!(Round { opponent_shape: Scissors, self_shape: Rock }, Round::from_str_with_prescribed_outcome("C Z").unwrap());
    }

    #[test]
    fn test_round_score() {
        assert_eq!(8, Round { opponent_shape: Rock, self_shape: Paper }.score());
        assert_eq!(1, Round { opponent_shape: Paper, self_shape: Rock }.score());
        assert_eq!(6, Round { opponent_shape: Scissors, self_shape: Scissors }.score());
    }
}
