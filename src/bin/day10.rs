use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let instructions: Vec<Instruction> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| Instruction::from_str(&line))
            .collect::<Result<_, _>>()?;

        let video_system = VideoSystem { instructions };

        let total_signal_strength: i32 = [20, 60, 100, 140, 180, 220].iter()
            .map(|&cycle| video_system.signal_strength(cycle))
            .sum();

        println!("Sum of signal strengths: {}", total_signal_strength);

        Ok(())
    } else {
        Err("Usage: day09 INPUT_FILE_PATH".into())
    }
}

struct VideoSystem {
    instructions: Vec<Instruction>,
}

impl VideoSystem {
    fn signal_strength(&self, cycle: i32) -> i32 {
        let mut current_cycle = 1;
        let mut x = 1;

        for instruction in &self.instructions {
            let (delta_cycle, delta_x) = match instruction {
                Instruction::Noop => (1, 0),
                Instruction::AddX(delta) => (2, *delta),
            };

            if current_cycle + delta_cycle > cycle {
                return x * cycle
            }

            current_cycle += delta_cycle;
            x += delta_x;
        }

        unreachable!()
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.split(' ').collect::<Vec<&str>>().as_slice() {
            ["noop"] => Ok(Instruction::Noop),
            ["addx", value] => Ok(Instruction::AddX(value.parse()?)),
            _ => Err("Unrecognized instruction".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const TEST_INSTRUCTIONS: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn test_signal_strength() {
        let instructions: Vec<Instruction> = TEST_INSTRUCTIONS.lines()
            .map(Instruction::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        let video_system = VideoSystem { instructions };

        assert_eq!(420, video_system.signal_strength(20));
        assert_eq!(1140, video_system.signal_strength(60));
        assert_eq!(1800, video_system.signal_strength(100));
        assert_eq!(2940, video_system.signal_strength(140));
        assert_eq!(2880, video_system.signal_strength(180));
        assert_eq!(3960, video_system.signal_strength(220));
    }
}
