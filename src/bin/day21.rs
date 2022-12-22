use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let monkeys = YellingMonkeys::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Monkey named 'root' yells: {}", monkeys.eval("root"));

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

struct YellingMonkeys {
    monkeys: HashMap<String, Monkey>,
}

impl YellingMonkeys {
    fn eval(&self, monkey_name: &str) -> u64 {
        match self.monkeys.get(monkey_name).unwrap() {
            Monkey::Literal(literal) => *literal,
            Monkey::Add(a, b) => self.eval(a) + self.eval(b),
            Monkey::Subtract(a, b) => self.eval(a) - self.eval(b),
            Monkey::Multiply(a, b) => self.eval(a) * self.eval(b),
            Monkey::Divide(a, b) => self.eval(a) / self.eval(b),
        }
    }
}

impl FromStr for YellingMonkeys {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut monkeys = HashMap::new();

        for line in string.lines() {
            if let [name, expression] = line.split(": ").collect::<Vec<&str>>().as_slice() {
                let monkey = match expression.split(' ').collect::<Vec<&str>>().as_slice() {
                    [literal] => Monkey::Literal(literal.parse()?),
                    [a, "+", b] => Monkey::Add(a.to_string(), b.to_string()),
                    [a, "-", b] => Monkey::Subtract(a.to_string(), b.to_string()),
                    [a, "*", b] => Monkey::Multiply(a.to_string(), b.to_string()),
                    [a, "/", b] => Monkey::Divide(a.to_string(), b.to_string()),
                    _ => return Err("Could not parse monkey line".into()),
                };

                monkeys.insert(name.to_string(), monkey);
            } else {
                return Err("Could not parse monkey line".into());
            }
        }

        Ok(YellingMonkeys { monkeys })
    }
}

enum Monkey {
    Literal(u64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MONKEYS: &str = indoc! {"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32"
    };

    #[test]
    fn test_eval() {
        let monkeys = YellingMonkeys::from_str(TEST_MONKEYS).unwrap();
        assert_eq!(152, monkeys.eval("root"));
    }
}
