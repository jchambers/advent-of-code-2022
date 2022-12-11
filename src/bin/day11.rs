use itertools::Itertools;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let monkey_group = MonkeyGroup::from_str(fs::read_to_string(path)?.as_str(), 3)?;

            println!(
                "Monkey business after 20 rounds with worry divisor of 3: {}",
                monkey_group.monkey_business(20)
            );
        }

        {
            let monkey_group = MonkeyGroup::from_str(fs::read_to_string(path)?.as_str(), 1)?;

            println!(
                "Monkey business after 10,000 rounds with worry divisor of 1: {}",
                monkey_group.monkey_business(10_000)
            );
        }

        Ok(())
    } else {
        Err("Usage: day11 INPUT_FILE_PATH".into())
    }
}

struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    worry_divisor: u64,
}

impl MonkeyGroup {
    fn from_str(string: &str, worry_divisor: u64) -> Result<Self, Box<dyn Error>> {
        let mut monkeys = vec![];

        for (empty, group) in &string.lines().group_by(|line| line.is_empty()) {
            if !empty {
                monkeys.push(Monkey::from_str(
                    group.intersperse("\n").collect::<String>().as_str(),
                )?);
            }
        }

        Ok(MonkeyGroup {
            monkeys,
            worry_divisor,
        })
    }

    fn group_modulus(&self) -> u64 {
        self.monkeys.iter().map(|monkey| monkey.modulus).product()
    }

    fn monkey_business(mut self, rounds: usize) -> u64 {
        let mut inspections = vec![0u64; self.monkeys.len()];

        for _ in 0..rounds {
            for m in 0..self.monkeys.len() {
                let monkey = &self.monkeys[m];

                let throws: Vec<(usize, u64)> = monkey
                    .items
                    .iter()
                    .map(|item| {
                        let worry_level = match monkey.operation {
                            Operation::Add(addend) => item + addend,
                            Operation::Multiply(multiplier) => item * multiplier,
                            Operation::Square => item * item,
                        } / self.worry_divisor;

                        let destination = match worry_level % monkey.modulus == 0 {
                            true => monkey.destinations[0],
                            false => monkey.destinations[1],
                        };

                        (destination, worry_level % self.group_modulus())
                    })
                    .collect();

                inspections[m] += throws.len() as u64;

                throws.into_iter().for_each(|(destination, worry_level)| {
                    self.monkeys[destination].items.push(worry_level)
                });

                self.monkeys[m].items.clear();
            }
        }

        inspections.sort();
        inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    modulus: u64,
    destinations: [usize; 2],
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut lines = string.lines();

        let _id: usize = lines
            .next()
            .and_then(|line| line.strip_prefix("Monkey "))
            .and_then(|line| line.strip_suffix(':'))
            .ok_or("No monkey ID line")?
            .parse()?;

        let items: Vec<u64> = lines
            .next()
            .and_then(|line| line.strip_prefix("  Starting items: "))
            .ok_or("No starting items line")?
            .split(", ")
            .map(|worry_level| worry_level.parse())
            .collect::<Result<_, _>>()?;

        let operation = match lines
            .next()
            .and_then(|line| line.strip_prefix("  Operation: new = "))
            .ok_or("No operation line")?
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["old", "+", addend] => Operation::Add(addend.parse()?),
            ["old", "*", "old"] => Operation::Square,
            ["old", "*", multiplier] => Operation::Multiply(multiplier.parse()?),
            _ => return Err("Could not parse operation".into()),
        };

        let modulus: u64 = lines
            .next()
            .and_then(|line| line.strip_prefix("  Test: divisible by "))
            .ok_or("No test line")?
            .parse()?;

        let true_destination: usize = lines
            .next()
            .and_then(|line| line.strip_prefix("    If true: throw to monkey "))
            .ok_or("No true destination line")?
            .parse()?;

        let false_destination: usize = lines
            .next()
            .and_then(|line| line.strip_prefix("    If false: throw to monkey "))
            .ok_or("No false destination line")?
            .parse()?;

        Ok(Monkey {
            items,
            operation,
            modulus,
            destinations: [true_destination, false_destination],
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MONKEYS: &str = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    #[test]
    fn test_monkey_business() {
        {
            let monkey_group = MonkeyGroup::from_str(TEST_MONKEYS, 3).unwrap();
            assert_eq!(10605, monkey_group.monkey_business(20));
        }

        {
            let monkey_group = MonkeyGroup::from_str(TEST_MONKEYS, 1).unwrap();
            assert_eq!(2713310158, monkey_group.monkey_business(10000));
        }
    }
}
