use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let values: Vec<i64> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;

        let mut gps = GrovePositioningSystem::try_from(values.as_slice())?;

        println!(
            "Coordinate sum with key = 1, mixing rounds = 1: {}",
            gps.coordinate_sum(1, 1)
        );

        println!(
            "Coordinate sum with key = 811589153, mixing rounds = 10: {}",
            gps.coordinate_sum(811589153, 10)
        );

        Ok(())
    } else {
        Err("Usage: day20 INPUT_FILE_PATH".into())
    }
}

struct GrovePositioningSystem {
    ring: *mut RingNode,
    len: usize,
}

impl GrovePositioningSystem {
    fn ring(&self) -> &RingNode {
        unsafe { &*(self.ring) }
    }

    unsafe fn mix(&mut self, decryption_key: i64) {
        let mut node = self.ring;

        loop {
            // `len - 1` in this case because the list no longer contains `node`
            let move_distance = ((*node).value * decryption_key) % (self.len - 1) as i64;

            if move_distance != 0 {
                // Remove `node` from its current position
                (*(*node).mixed_prev).mixed_next = (*node).mixed_next;
                (*(*node).mixed_next).mixed_prev = (*node).mixed_prev;

                let (left, right) = if move_distance > 0 {
                    let mut left = node;

                    for _ in 0..move_distance {
                        left = (*left).mixed_next;
                    }

                    (left, (*left).mixed_next)
                } else {
                    let mut right = node;

                    for _ in 0..move_distance.abs() {
                        right = (*right).mixed_prev;
                    }

                    ((*right).mixed_prev, right)
                };

                // Add `node` to its new position
                (*left).mixed_next = node;
                (*right).mixed_prev = node;
                (*node).mixed_prev = left;
                (*node).mixed_next = right;
            }

            node = (*node).original_next;

            if std::ptr::eq(node, self.ring) {
                break;
            }
        }
    }

    fn reset(&mut self) {
        let mut node = self.ring;

        loop {
            unsafe {
                (*node).mixed_next = (*node).original_next;
                (*node).mixed_prev = (*node).original_prev;

                node = (*node).original_next;
            }

            if std::ptr::eq(node, self.ring) {
                break;
            }
        }
    }

    fn coordinate_sum(&mut self, decryption_key: i64, mixing_rounds: usize) -> i64 {
        self.reset();

        for _ in 0..mixing_rounds {
            unsafe {
                self.mix(decryption_key);
            }
        }

        let mut node = {
            let mut node = self.ring();

            loop {
                if node.value == 0 {
                    break node;
                }

                node = node.original_next();
            }
        };

        let mut sum = 0;

        for _ in 0..3 {
            for _ in 0..1000 {
                node = node.mixed_next();
            }

            sum += node.value * decryption_key;
        }

        sum
    }
}

impl TryFrom<&[i64]> for GrovePositioningSystem {
    type Error = Box<dyn Error>;

    fn try_from(values: &[i64]) -> Result<Self, Self::Error> {
        if values.is_empty() {
            Err("Values must not be empty".into())
        } else {
            let mut head = Box::into_raw(Box::new(RingNode::new(values[0])));

            unsafe {
                (*head).original_next = head;
                (*head).original_prev = head;
                (*head).mixed_next = head;
                (*head).mixed_prev = head;
            }

            for i in &values[1..] {
                let tail = unsafe { (*head).original_prev };

                let new_tail = Box::into_raw(Box::new(RingNode {
                    original_next: head,
                    original_prev: tail,
                    mixed_next: head,
                    mixed_prev: tail,
                    value: *i,
                }));

                unsafe {
                    (*head).original_prev = new_tail;
                    (*head).mixed_prev = new_tail;
                    (*tail).original_next = new_tail;
                    (*tail).mixed_next = new_tail;
                }
            }

            let gps = Self {
                ring: head,
                len: values.len(),
            };

            Ok(gps)
        }
    }
}

impl RingNode {
    fn new(value: i64) -> Self {
        RingNode {
            original_next: std::ptr::null_mut(),
            original_prev: std::ptr::null_mut(),

            mixed_next: std::ptr::null_mut(),
            mixed_prev: std::ptr::null_mut(),

            value,
        }
    }
}

struct RingNode {
    original_next: *mut RingNode,
    original_prev: *mut RingNode,

    mixed_next: *mut RingNode,
    mixed_prev: *mut RingNode,

    value: i64,
}

impl RingNode {
    fn original_next(&self) -> &RingNode {
        unsafe { &*(self.original_next) }
    }

    fn mixed_next(&self) -> &RingNode {
        unsafe { &*(self.mixed_next) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_NUMBERS: [i64; 7] = [1, 2, -3, 3, -2, 0, 4];

    #[test]
    fn test_mix() {
        let mut gps = GrovePositioningSystem::try_from(TEST_NUMBERS.as_slice()).unwrap();

        unsafe {
            gps.mix(1);
        }

        {
            let mut ring = gps.ring();
            let mut values = vec![];

            loop {
                values.push(ring.value);
                ring = ring.original_next();

                if std::ptr::eq(ring, gps.ring()) {
                    break;
                }
            }

            assert_eq!(Vec::from(TEST_NUMBERS), values);
        };

        {
            let mut ring = gps.ring();
            let mut values = vec![];

            loop {
                values.push(ring.value);
                ring = ring.mixed_next();

                if std::ptr::eq(ring, gps.ring()) {
                    break;
                }
            }

            assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], values);
        };
    }

    #[test]
    fn test_coordinate_sum() {
        let mut gps = GrovePositioningSystem::try_from(TEST_NUMBERS.as_slice()).unwrap();
        assert_eq!(3, gps.coordinate_sum(1, 1));
        assert_eq!(1623178306, gps.coordinate_sum(811589153, 10));
    }
}
