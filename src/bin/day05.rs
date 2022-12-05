use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let crate_stacks = CrateStacks::from_str(fs::read_to_string(path)?.as_str())?;

            println!(
                "Top crates after applying instructions one crate at a time: {}",
                crate_stacks.top_crates_after_instructions_individual()
            );
        }

        {
            let crate_stacks = CrateStacks::from_str(fs::read_to_string(path)?.as_str())?;

            println!(
                "Top crates after applying instructions to groups of crates: {}",
                crate_stacks.top_crates_after_instructions_group()
            );
        }

        Ok(())
    } else {
        Err("Usage: day05 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug)]
struct CrateStacks {
    stacks: Vec<VecDeque<char>>,
    instructions: Vec<Instruction>,
}

impl FromStr for CrateStacks {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // First, process the initial crate positions
        let mut stacks = {
            let max_line_length = string.lines()
                .take_while(|line| line.contains('['))
                .map(|line| line.len())
                .max()
                .unwrap();

            vec![VecDeque::new(); (max_line_length + 1) / 4]
        };

        string.lines()
            .take_while(|line| line.contains('['))
            .for_each(|line| {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .for_each(|(i, c)| {
                        stacks[i].push_front(c);
                    });
            });

        // …and then onto the instructions
        let instructions = string.lines()
            .filter(|line| line.starts_with("move"))
            .map(Instruction::from_str)
            .collect::<Result<Vec<Instruction>, _>>()?;

        Ok(CrateStacks { stacks, instructions })
    }
}

impl CrateStacks {
    fn top_crates_after_instructions_individual(mut self) -> String {
        self.instructions.iter()
            .for_each(|instruction| {
                for _ in 0..instruction.quantity {
                    let c = self.stacks[instruction.source - 1].pop_back().expect("Stack should not be empty");
                    self.stacks[instruction.destination - 1].push_back(c);
                }
            });

        self.top_crates()
    }

    fn top_crates_after_instructions_group(mut self) -> String {
        self.instructions.iter()
            .for_each(|instruction| {
                let stack_len = self.stacks[instruction.source - 1].len();
                let mut moved_crates =
                    self.stacks[instruction.source - 1].split_off(stack_len - instruction.quantity);

                self.stacks[instruction.destination - 1].append(&mut moved_crates);
            });

        self.top_crates()
    }

    fn top_crates(self) -> String {
        self.stacks.iter()
            .map(|stack| stack.back().unwrap_or(&' '))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    quantity: usize,
    source: usize,
    destination: usize,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_PATTERN: Regex =
                    Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        if let Some(captures) = INSTRUCTION_PATTERN.captures(string) {
            let quantity = captures[1].parse()?;
            let source = captures[2].parse()?;
            let destination = captures[3].parse()?;

            Ok( Instruction { quantity, source, destination })
        } else {
            Err("Could not parse instruction string".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_STACKS: &str = indoc! {"
             [D]
         [N] [C]
         [Z] [M] [P]
          1   2   3

         move 1 from 2 to 1
         move 3 from 1 to 3
         move 2 from 2 to 1
         move 1 from 1 to 2
    "};

    #[test]
    fn test_instruction_from_string() {
        assert_eq!(
            Instruction {
                quantity: 12,
                source: 77,
                destination: 49,
            },
            Instruction::from_str("move 12 from 77 to 49").unwrap()
        )
    }

    #[test]
    fn test_top_crates_after_instructions_individual() {
        let stacks = CrateStacks::from_str(TEST_STACKS).unwrap();
        assert_eq!("CMZ", stacks.top_crates_after_instructions_individual());
    }

    #[test]
    fn test_top_crates_after_instructions_group() {
        let stacks = CrateStacks::from_str(TEST_STACKS).unwrap();
        assert_eq!("MCD", stacks.top_crates_after_instructions_group());
    }
}
