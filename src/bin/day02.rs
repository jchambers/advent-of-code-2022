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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl TryFrom<usize> for Shape {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rock),
            1 => Ok(Paper),
            2 => Ok(Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn try_from_opponent_shape_str(string: &str) -> Result<Self, Box<dyn Error>> {
        match string {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            _ => Err("Unrecognized opponent shape".into())
        }
    }

    fn value(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    /// Returns the shape a player must choose to win a round in which an opponent plays this shape
    fn winning_move(&self) -> Shape {
        Shape::try_from((*self as usize + 1) % 3).unwrap()
    }

    /// Returns the shape a player must choose to lose a round in which an opponent plays this shape
    fn losing_move(&self) -> Shape {
        Shape::try_from((*self as usize + 2) % 3).unwrap()
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

        let opponent_shape = Shape::try_from_opponent_shape_str(&string[0..1])?;
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

        let opponent_shape = Shape::try_from_opponent_shape_str(&string[0..1])?;
        let self_shape = match &string[2..] {
            "X" => opponent_shape.losing_move(),
            "Y" => opponent_shape,
            "Z" => opponent_shape.winning_move(),
            _ => return Err("Unrecognized self shape".into())
        };

        Ok(Round { self_shape, opponent_shape })
    }

    fn score(&self) -> u32 {
        let outcome = match (self.opponent_shape as isize - self.self_shape as isize + 3) % 3 {
            0 => Draw,
            1 => Lose,
            2 => Win,
            _ => unreachable!(),
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

    #[test]
    fn test_winning_move() {
        assert_eq!(Paper, Rock.winning_move());
        assert_eq!(Scissors, Paper.winning_move());
        assert_eq!(Rock, Scissors.winning_move());
    }

    #[test]
    fn test_losing_move() {
        assert_eq!(Scissors, Rock.losing_move());
        assert_eq!(Rock, Paper.losing_move());
        assert_eq!(Paper, Scissors.losing_move());
    }
}
