use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let numbers: Vec<SnafuNumber> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| SnafuNumber::from_str(line.as_str()))
            .collect::<Result<_, _>>()?;

        let sum: i64 = numbers
            .iter()
            .map(i64::from)
            .sum();

        println!("Sum of SNAFU numbers: {}", SnafuNumber::from(sum).to_string());

        Ok(())
    } else {
        Err("Usage: day25 INPUT_FILE_PATH".into())
    }
}

struct SnafuNumber {
    snafu_digits: Vec<i8>,
}

impl SnafuNumber {
    fn max(digits: usize) -> i64 {
        (0..digits)
            .map(|place| 2 * 5i64.pow(place as u32))
            .sum()
    }

    fn min(digits: usize) -> i64 {
        -SnafuNumber::max(digits)
    }
}

impl FromStr for SnafuNumber {
    type Err = Box<dyn Error>;

    fn from_str(snafu_digits: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            snafu_digits: snafu_digits
                .chars()
                .map(|c| match c {
                    '2' => Ok(2),
                    '1' => Ok(1),
                    '0' => Ok(0),
                    '-' => Ok(-1),
                    '=' => Ok(-2),
                    _ => Err("Unexpected character".into()),
                })
                .collect::<Result<_, Box<dyn Error>>>()?
        })
    }
}

impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        self.snafu_digits
            .iter()
            .map(|digit| match digit {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            })
            .collect()
    }
}

impl From<SnafuNumber> for i64 {
    fn from(snafu_number: SnafuNumber) -> Self {
        i64::from(&snafu_number)
    }
}

impl From<&SnafuNumber> for i64 {
    fn from(snafu_number: &SnafuNumber) -> Self {
        snafu_number.snafu_digits
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (place, digit)| {
                acc + (*digit as i64 * 5i64.pow(place as u32))
            })
    }
}

impl From<i64> for SnafuNumber {
    fn from(decimal: i64) -> Self {
        let mut digits = {
            let mut digits_required = 0;

            loop {
                digits_required += 1;

                let min = 5i64.pow(digits_required - 1) + SnafuNumber::min(digits_required as usize - 1);
                let max = 2 * 5i64.pow(digits_required - 1) + SnafuNumber::max(digits_required as usize - 1);

                if decimal >= min && decimal <= max {
                    break;
                }
            };

            vec![0i8; digits_required as usize]
        };

        let mut remainder = decimal;

        for place in (0..digits.len()).rev() {
            for candidate in -2..=2 {
                let min = candidate * 5i64.pow(place as u32) + SnafuNumber::min(place);
                let max = candidate * 5i64.pow(place as u32) + SnafuNumber::max(place);

                if remainder >= min && remainder <= max {
                    digits[place] = candidate as i8;
                    remainder -= candidate * 5i64.pow(place as u32);
                    break;
                }
            }
        }

        SnafuNumber { snafu_digits: digits.iter().rev().copied().collect() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        for (snafu, decimal) in [
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ] {
            assert_eq!(decimal as i64, SnafuNumber::from_str(snafu).unwrap().into());
        }
    }

    #[test]
    fn test_decimal_to_snafu() {
        for (decimal, snafu) in [
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ] {
            assert_eq!(snafu, SnafuNumber::from(decimal).to_string());
        }
    }
}
