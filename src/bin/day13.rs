use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let packets = fs::read_to_string(path)?;

        println!(
            "Sum of indices of correctly-ordered pairs: {}",
            Packet::index_sum_of_correctly_ordered_pairs(&packets)?
        );

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Packet {
    value: Value,
}

impl FromStr for Packet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];
        let chars_vec = s.chars().collect::<Vec<char>>();
        let mut chars = chars_vec.as_slice();

        while !chars.is_empty() {
            let offset = match chars[0] {
                '[' => {
                    stack.push(vec![]);
                    1
                }

                ']' => {
                    let value = Value::List(stack.pop().ok_or("Tried to pop empty stack")?);

                    if stack.is_empty() {
                        return Ok(Packet { value });
                    } else {
                        stack
                            .last_mut()
                            .expect("Stack should not be empty")
                            .push(value);
                    }

                    1
                }

                ',' => 1,

                '0'..='9' => {
                    let digits = chars
                        .iter()
                        .take_while(|c| c.is_numeric())
                        .collect::<String>();

                    stack
                        .last_mut()
                        .ok_or("Tried to append numeric value to empty stack")?
                        .push(Value::Integer(digits.parse()?));

                    digits.len()
                }

                _ => {
                    return Err("Unexpected character".into());
                }
            };

            chars = &chars[offset..];
        }

        Err("Unexpected end of string".into())
    }
}

impl Packet {
    fn index_sum_of_correctly_ordered_pairs(packets: &str) -> Result<usize, Box<dyn Error>> {
        packets
            .split("\n\n")
            .enumerate()
            .try_fold(0, |acc, (i, pair)| {
                if let [a, b] = pair.split('\n').collect::<Vec<&str>>().as_slice() {
                    if Packet::from_str(a)? < Packet::from_str(b)? {
                        Ok(acc + i + 1)
                    } else {
                        Ok(acc)
                    }
                } else {
                    Err("Could not parse packet pair".into())
                }
            })
    }
}

enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),

            (Value::List(a), Value::List(b)) => {
                let mut i = 0;

                loop {
                    match (a.get(i), b.get(i)) {
                        (Some(v_a), Some(v_b)) => {
                            if v_a != v_b {
                                break v_a.cmp(v_b);
                            }
                        }
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (None, None) => break Ordering::Equal,
                    };

                    i += 1;
                }
            }

            (Value::Integer(a), Value::List(_)) => Value::List(vec![Value::Integer(*a)]).cmp(other),

            (Value::List(_), Value::Integer(b)) => self.cmp(&Value::List(vec![Value::Integer(*b)])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Value {}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_PACKETS: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]"
    };

    #[test]
    fn test_value_compare() {
        assert_eq!(Ordering::Less, Value::Integer(1).cmp(&Value::Integer(2)));
        assert_eq!(Ordering::Equal, Value::Integer(1).cmp(&Value::Integer(1)));
        assert_eq!(Ordering::Greater, Value::Integer(2).cmp(&Value::Integer(1)));

        assert_eq!(
            Ordering::Less,
            Value::List(vec![Value::Integer(1)]).cmp(&Value::List(vec![Value::Integer(2)]))
        );

        assert_eq!(
            Ordering::Equal,
            Value::List(vec![Value::Integer(1)]).cmp(&Value::List(vec![Value::Integer(1)]))
        );

        assert_eq!(
            Ordering::Greater,
            Value::List(vec![Value::Integer(2)]).cmp(&Value::List(vec![Value::Integer(1)]))
        );

        assert_eq!(
            Ordering::Less,
            Value::List(vec![Value::Integer(1)])
                .cmp(&Value::List(vec![Value::Integer(1), Value::Integer(2)]))
        );

        assert_eq!(
            Ordering::Greater,
            Value::List(vec![Value::Integer(2), Value::Integer(1)])
                .cmp(&Value::List(vec![Value::Integer(1)]))
        );

        assert_eq!(
            Ordering::Equal,
            Value::List(vec![Value::Integer(1)]).cmp(&Value::Integer(1))
        );

        assert_eq!(
            Ordering::Equal,
            Value::Integer(1).cmp(&Value::List(vec![Value::Integer(1)]))
        );

        assert_eq!(
            Ordering::Less,
            Value::List(vec![
                Value::Integer(0),
                Value::Integer(0),
                Value::Integer(0)
            ])
            .cmp(&Value::Integer(2))
        );
    }

    #[test]
    fn test_packet_compare() {
        assert_eq!(
            Ordering::Less,
            Packet::from_str("[1,1,3,1,1]")
                .unwrap()
                .cmp(&Packet::from_str("[1,1,5,1,1]").unwrap())
        );

        assert_eq!(
            Ordering::Less,
            Packet::from_str("[[1],[2,3,4]]")
                .unwrap()
                .cmp(&Packet::from_str("[[1],4]").unwrap())
        );

        assert_eq!(
            Ordering::Greater,
            Packet::from_str("[9]")
                .unwrap()
                .cmp(&Packet::from_str("[[8,7,6]]").unwrap())
        );

        assert_eq!(
            Ordering::Less,
            Packet::from_str("[[4,4],4,4]")
                .unwrap()
                .cmp(&Packet::from_str("[[4,4],4,4,4]").unwrap())
        );

        assert_eq!(
            Ordering::Greater,
            Packet::from_str("[7,7,7,7]")
                .unwrap()
                .cmp(&Packet::from_str("[7,7,7]").unwrap())
        );

        assert_eq!(
            Ordering::Less,
            Packet::from_str("[]")
                .unwrap()
                .cmp(&Packet::from_str("[3]").unwrap())
        );

        assert_eq!(
            Ordering::Greater,
            Packet::from_str("[[[]]]")
                .unwrap()
                .cmp(&Packet::from_str("[[]]").unwrap())
        );

        assert_eq!(
            Ordering::Greater,
            Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]")
                .unwrap()
                .cmp(&Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap())
        );
    }

    #[test]
    fn test_index_sum_of_correctly_ordered_pairs() {
        assert_eq!(
            13,
            Packet::index_sum_of_correctly_ordered_pairs(TEST_PACKETS).unwrap()
        );
    }
}
