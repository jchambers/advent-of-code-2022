use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let monkeys = YellingMonkeys::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Monkey named 'root' yells: {}", monkeys.eval("root"));
        println!("Human should yell: {}", monkeys.find_human_number());

        Ok(())
    } else {
        Err("Usage: day21 INPUT_FILE_PATH".into())
    }
}

struct YellingMonkeys {
    monkeys: HashMap<String, Monkey>,
}

impl YellingMonkeys {
    fn eval(&self, monkey_name: &str) -> f64 {
        match self.monkeys.get(monkey_name).unwrap() {
            Monkey::Literal(literal) => *literal,
            Monkey::Add(a, b) => self.eval(a) + self.eval(b),
            Monkey::Subtract(a, b) => self.eval(a) - self.eval(b),
            Monkey::Multiply(a, b) => self.eval(a) * self.eval(b),
            Monkey::Divide(a, b) => self.eval(a) / self.eval(b),
        }
    }

    fn find_human_number(mut self) -> f64 {
        const ROOT: &str = "root";

        let monkeys = match self.monkeys.get(ROOT).unwrap() {
            Monkey::Literal(_) => panic!("Root has a literal value"),
            Monkey::Add(a, b) => (a.clone(), b.clone()),
            Monkey::Subtract(a, b) => (a.clone(), b.clone()),
            Monkey::Multiply(a, b) => (a.clone(), b.clone()),
            Monkey::Divide(a, b) => (a.clone(), b.clone()),
        };

        let mut left = 0f64;
        let mut right = {
            let delta_left = self.delta(left, &monkeys);
            let mut right = 1f64;

            while self.delta(right, &monkeys).signum() == delta_left.signum() {
                right *= 2f64;
            }

            right
        };

        loop {
            let mid = (left + right) / 2f64;

            let a = self.delta(left as f64, &monkeys);
            let b = self.delta(right, &monkeys);
            let c = self.delta(mid, &monkeys);

            if a == 0f64 {
                return left;
            } else if b == 0f64 {
                return right;
            } else if c == 0f64 {
                return mid;
            }

            if a.signum() != c.signum() {
                right = mid;
            } else if b.signum() != c.signum() {
                left = mid;
            } else {
                panic!("All values have same sign")
            }
        }
    }

    fn delta(&mut self, value: f64, monkeys: &(String, String)) -> f64 {
        const HUMAN: &str = "humn";

        *self.monkeys.get_mut(HUMAN).unwrap() = Monkey::Literal(value);
        self.eval(monkeys.0.as_str()) as f64 - self.eval(monkeys.1.as_str()) as f64
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
    Literal(f64),
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
        assert_eq!(152f64, monkeys.eval("root"));
    }

    #[test]
    fn test_find_human_number() {
        let monkeys = YellingMonkeys::from_str(TEST_MONKEYS).unwrap();
        assert_eq!(301f64, monkeys.find_human_number());
    }
}
