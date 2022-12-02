use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::RoundOutcome::{Draw, Lose, Win};
use crate::Shape::{Paper, Rock, Scissors};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let rounds: Vec<Round> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| Round::from_str(&line).ok())
            .collect();

        println!("Total score: {}", rounds.iter().map(|round| round.score()).sum::<u32>());

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

impl FromStr for Round {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
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
            _ => return Err("Unrecognized opponent shape".into())
        };

        Ok(Round { self_shape, opponent_shape })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_from_string() {
        assert_eq!(Round { opponent_shape: Rock, self_shape: Paper }, Round::from_str("A Y").unwrap());
        assert_eq!(Round { opponent_shape: Paper, self_shape: Rock }, Round::from_str("B X").unwrap());
        assert_eq!(Round { opponent_shape: Scissors, self_shape: Scissors }, Round::from_str("C Z").unwrap());
    }

    #[test]
    fn test_round_score() {
        assert_eq!(8, Round { opponent_shape: Rock, self_shape: Paper }.score());
        assert_eq!(1, Round { opponent_shape: Paper, self_shape: Rock }.score());
        assert_eq!(6, Round { opponent_shape: Scissors, self_shape: Scissors }.score());
    }
}
