use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let sensors: Vec<Sensor> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| Sensor::from_str(&line))
            .collect::<Result<_, _>>()
            .unwrap();

        let sensor_array = SensorArray { sensors };

        println!(
            "Positions without beacon in row 2,000,000: {}",
            sensor_array.positions_without_beacon(2_000_000)
        );

        println!(
            "Tuning frequency with x/y <= 4,000,000: {}",
            sensor_array.tuning_frequency(4_000_000).unwrap()
        );

        Ok(())
    } else {
        Err("Usage: day15 INPUT_FILE_PATH".into())
    }
}

struct SensorArray {
    sensors: Vec<Sensor>,
}

impl SensorArray {
    fn beacon_positions(&self) -> HashSet<(i32, i32)> {
        self.sensors
            .iter()
            .map(|sensor| (sensor.closest_beacon_x, sensor.closest_beacon_y))
            .collect()
    }

    fn covered_ranges(&self, y: i32) -> Vec<Range> {
        let mut ranges: Vec<Range> = self
            .sensors
            .iter()
            .filter_map(|sensor| sensor.covered_range(y))
            .collect();

        ranges.sort_by_key(|range| range.start);

        let mut merged_ranges: Vec<Range> = vec![];

        for range in ranges {
            if let Some(merged_range) = merged_ranges
                .last()
                .and_then(|tail_range| tail_range.union(&range))
            {
                merged_ranges.pop();
                merged_ranges.push(merged_range);
            } else {
                merged_ranges.push(range);
            }
        }

        merged_ranges
    }

    fn positions_without_beacon(&self, y: i32) -> u32 {
        self.covered_ranges(y)
            .iter()
            .map(|range| range.span())
            .sum::<u32>()
            - self
                .beacon_positions()
                .iter()
                .filter(|(_, beacon_y)| *beacon_y == y)
                .count() as u32
    }

    fn tuning_frequency(&self, max_coordinate: i32) -> Option<u64> {
        let bounds = Range::new(0, max_coordinate);

        for y in 0..=max_coordinate {
            let covered_ranges = self.covered_ranges(y);

            let scanned_spaces: u32 = covered_ranges
                .iter()
                .filter_map(|range| {
                    range
                        .intersection(&bounds)
                        .map(|intersection| intersection.span())
                })
                .sum();

            // A more verbose way to write this would be:
            //
            // ```
            // let desired_positions_without_beacon = Range::new(0, max_coordinate).span() - 1;
            // ```
            //
            // …but that works out to just be `max_coordinate` in practice.
            if scanned_spaces == max_coordinate as u32 {
                let x = if covered_ranges[0].start == 1 {
                    // Literal edge case; the only uncovered spot is at x = 0
                    0
                } else if covered_ranges[covered_ranges.len() - 1].end == max_coordinate - 1 {
                    // Edge case on the far end; the only uncovered spot is at the very end of the
                    // row
                    max_coordinate - 1
                } else {
                    covered_ranges
                        .windows(2)
                        .find(|pair| pair[1].start == pair[0].end + 2)
                        .map(|pair| pair[0].end + 1)
                        .expect("Should have two intervals with a one-space gap")
                };

                return Some((x as u64 * 4000000) + y as u64);
            }
        }

        None
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    x: i32,
    y: i32,

    closest_beacon_x: i32,
    closest_beacon_y: i32,
}

impl FromStr for Sensor {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref SENSOR_PATTERN: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }

        if let Some(captures) = SENSOR_PATTERN.captures(string) {
            let x = captures[1].parse()?;
            let y = captures[2].parse()?;
            let closest_beacon_x = captures[3].parse()?;
            let closest_beacon_y = captures[4].parse()?;

            Ok(Sensor {
                x,
                y,
                closest_beacon_x,
                closest_beacon_y,
            })
        } else {
            Err("Could not parse instruction string".into())
        }
    }
}

impl Sensor {
    fn radius(&self) -> u32 {
        self.x.abs_diff(self.closest_beacon_x) + self.y.abs_diff(self.closest_beacon_y)
    }

    fn covered_range(&self, y: i32) -> Option<Range> {
        if self.y.abs_diff(y) <= self.radius() {
            Some(Range {
                start: self.x - (self.radius() - self.y.abs_diff(y)) as i32,
                end: self.x + (self.radius() - self.y.abs_diff(y)) as i32,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }

    fn span(&self) -> u32 {
        self.end.abs_diff(self.start) + 1
    }

    fn union(&self, other: &Self) -> Option<Self> {
        if other.end >= self.start && other.start <= self.end {
            Some(Range::new(
                self.start.min(other.start),
                self.end.max(other.end),
            ))
        } else {
            None
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if other.end >= self.start && other.start <= self.end {
            Some(Range::new(
                self.start.max(other.start),
                self.end.min(other.end),
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_SENSORS: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    };

    #[test]
    fn test_sensor_from_string() {
        let sensor =
            Sensor::from_str("Sensor at x=2, y=18: closest beacon is at x=-2, y=15").unwrap();

        assert_eq!(
            Sensor {
                x: 2,
                y: 18,
                closest_beacon_x: -2,
                closest_beacon_y: 15,
            },
            sensor
        );
    }

    #[test]
    fn test_covered_range() {
        let sensor =
            Sensor::from_str("Sensor at x=8, y=7: closest beacon is at x=2, y=10").unwrap();

        assert_eq!(Some(Range::new(-1, 17)), sensor.covered_range(7));
        assert_eq!(Some(Range::new(8, 8)), sensor.covered_range(16));
        assert_eq!(None, sensor.covered_range(17));
    }

    #[test]
    fn test_positions_without_beacon() {
        let sensors: Vec<Sensor> = TEST_SENSORS
            .lines()
            .map(Sensor::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        let sensor_array = SensorArray { sensors };

        assert_eq!(26, sensor_array.positions_without_beacon(10));
    }

    #[test]
    fn test_tuning_frequency() {
        let sensors: Vec<Sensor> = TEST_SENSORS
            .lines()
            .map(Sensor::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        let sensor_array = SensorArray { sensors };

        assert_eq!(Some(56000011), sensor_array.tuning_frequency(20));
    }
}
