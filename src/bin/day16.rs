extern crate core;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let mut volcano = Volcano::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Maximum pressure released over 30 minutes with 1 actor: {}",
            volcano.maximum_pressure_release(1, 30)
        );

        println!(
            "Maximum pressure released over 26 minutes with 2 actors: {}",
            volcano.maximum_pressure_release(2, 26)
        );

        Ok(())
    } else {
        Err("Usage: day16 INPUT_FILE_PATH".into())
    }
}

struct Volcano {
    flow_rates: HashMap<String, u32>,
    travel_times: HashMap<(String, String), u32>,
}

impl FromStr for Volcano {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref VALVE_PATTERN: Regex = Regex::new(
                r"Valve ([A-Z]+) has flow rate=(\d+); (?:tunnel|tunnels) (?:lead|leads) to (?:valve|valves) (.+)"
            )
            .unwrap();
        }

        let mut flow_rates = HashMap::new();
        let mut connections = HashMap::new();

        for line in string.lines().filter(|line| !line.is_empty()) {
            if let Some(captures) = VALVE_PATTERN.captures(line) {
                let valve = captures[1].to_string();
                let flow_rate = captures[2].parse()?;
                let destinations: Vec<String> = captures[3]
                    .split(", ")
                    .map(|connection| connection.to_string())
                    .collect();

                flow_rates.insert(valve.clone(), flow_rate);
                connections.insert(valve, destinations);
            } else {
                return Err("Could not parse line".into());
            }
        }

        // Precalculate travel times
        let valves = {
            let mut valves: Vec<String> = flow_rates
                .iter()
                .filter_map(|(valve, flow_rate)| if *flow_rate > 0 {
                    Some(valve.clone())
                } else {
                    None
                })
                .collect();

            if !valves.contains(&"AA".to_string()) {
                valves.push("AA".to_string());
            }

            valves
        };

        let mut travel_times = HashMap::new();

        for a in 0..valves.len() - 1 {
            for b in a + 1..valves.len() {
                let mut tentative_distances = HashMap::from([(valves[a].to_string(), 0)]);
                let mut visited_valves: HashSet<String> = HashSet::new();

                while !visited_valves.contains(&valves[b]) {
                    let (valve, distance) = tentative_distances
                        .iter()
                        .filter(|(valve, _)| !visited_valves.contains(*valve))
                        .min_by_key(|(_, distance)| *distance)
                        .unwrap();

                    let valve = valve.clone();
                    let distance = *distance;

                    for neighbor in connections
                        .get(&valve)
                        .expect("Valve should have neighbors")
                    {
                        let neighbor_distance = tentative_distances
                            .entry(neighbor.clone())
                            .or_insert(u32::MAX);

                        *neighbor_distance = (*neighbor_distance).min(distance + 1);
                    }

                    visited_valves.insert(valve.clone());
                }


                let distance = *tentative_distances.get(&valves[b]).unwrap();

                travel_times.insert((valves[a].clone(), valves[b].clone()), distance);
                travel_times.insert((valves[b].clone(), valves[a].clone()), distance);
            }
        }

        Ok(Volcano {
            flow_rates,
            travel_times,
        })
    }
}

impl Volcano {
    fn maximum_pressure_release(&mut self, actors: u32, time_limit: u32) -> u32 {
        const START: &str = "AA";

        let mut unopened_valves: HashSet<String> = self
            .flow_rates
            .iter()
            .filter_map(|(valve, flow_rate)| {
                if *flow_rate > 0 {
                    Some(valve.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut exploration_stack = vec![ExplorationAction::Backtrack];

        unopened_valves
            .iter()
            .filter(|valve| self.can_open_valve(START, valve, 0, time_limit))
            .for_each(|unopened_valve| {
                for actor in 0..actors {
                    exploration_stack.push(ExplorationAction::Explore(actor, unopened_valve.clone()));
                }
            });

        let mut path: Vec<(u32, u32, String)> = vec![];
        let mut max_pressure_released = 0;

        while !exploration_stack.is_empty() {
            match exploration_stack.pop().unwrap() {
                ExplorationAction::Explore(actor, valve) => {
                    let mut actor_states: Vec<(u32, String)> = (0..actors)
                        .map(|x| path
                            .iter()
                            .filter(|(a, _, _)| a == &x)
                            .last()
                            .map(|(_, time, valve)| (*time, valve.clone()))
                            .unwrap_or_else(|| (0, START.to_string())))
                        .collect();

                    let (previous_time, previous_valve) = &actor_states[actor as usize];
                    let travel_time = self.travel_cost(previous_valve, &valve);
                    let current_time = previous_time + travel_time + 1;

                    path.push((actor, current_time, valve.clone()));
                    unopened_valves.remove(valve.as_str());

                    actor_states[actor as usize] = (current_time, valve);

                    let mut next_actions = vec![];

                    for a in 0..actors {
                        let (time, valve) = &actor_states[a as usize];

                        unopened_valves
                            .iter()
                            .filter(|candidate| self.can_open_valve(&valve, candidate, *time, time_limit))
                            .for_each(|candidate| next_actions.push(ExplorationAction::Explore(a, candidate.clone())));
                    }

                    exploration_stack.push(ExplorationAction::Backtrack);

                    if next_actions.is_empty() {
                        // We've reached the end of the line
                        max_pressure_released = max_pressure_released
                            .max(self.pressure_released(path.as_slice(), time_limit));
                    } else {
                        exploration_stack.append(&mut next_actions);
                    }
                }
                ExplorationAction::Backtrack => {
                    path.pop()
                        .map(|(_, _, popped_valve)| unopened_valves.insert(popped_valve));
                }
            }
        }

        max_pressure_released
    }

    fn can_open_valve(&mut self, start: &str, valve: &str, current_time: u32, time_limit: u32) -> bool {
        current_time + self.travel_cost(start, valve) <= time_limit - 2
    }

    fn travel_cost(&mut self, start: &str, end: &str) -> u32 {
        if start == end {
            0
        } else {
            *self.travel_times.get(&(start.to_string(), end.to_string())).unwrap()
        }
    }

    fn pressure_released(&self, sequence: &[(u32, u32, String)], time_limit: u32) -> u32 {
        sequence
            .iter()
            .map(|(_, time, valve)| {
                self.flow_rates.get(valve).expect("Valve should be present") * (time_limit - time)
            })
            .sum()
    }
}

#[derive(Debug)]
enum ExplorationAction {
    Explore(u32, String),
    Backtrack,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_VALVES: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn test_travel_cost() {
        let mut volcano = Volcano::from_str(TEST_VALVES).unwrap();

        assert_eq!(0, volcano.travel_cost("AA", "AA"));
        assert_eq!(1, volcano.travel_cost("AA", "BB"));
        assert_eq!(2, volcano.travel_cost("AA", "CC"));
    }

    #[test]
    fn test_maximum_pressure_release() {
        let mut volcano = Volcano::from_str(TEST_VALVES).unwrap();

        assert_eq!(1651, volcano.maximum_pressure_release(1, 30));
        assert_eq!(1707, volcano.maximum_pressure_release(2, 26));
    }

    #[test]
    fn test_pressure_released() {
        let volcano = Volcano::from_str(TEST_VALVES).unwrap();

        assert_eq!(
            1651,
            volcano.pressure_released(
                vec![
                    (0, 2, "DD".to_string()),
                    (0, 5, "BB".to_string()),
                    (0, 9, "JJ".to_string()),
                    (0, 17, "HH".to_string()),
                    (0, 21, "EE".to_string()),
                    (0, 24, "CC".to_string()),
                ]
                .as_slice(),
                30
            )
        );
    }
}
