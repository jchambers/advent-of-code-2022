use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::ops::{Add, Sub};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let factory = RobotFactory::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Sum of quality levels of blueprints: {}",
            factory.quality_level_sum()
        );

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

struct RobotFactory {
    blueprints: Vec<Blueprint>,
}

impl RobotFactory {
    fn quality_level_sum(&self) -> u32 {
        self.blueprints
            .iter()
            .map(|blueprint| blueprint.optimize_geodes(24) as u32)
            .enumerate()
            .map(|(i, geodes)| (i as u32 + 1) * geodes)
            .sum()
    }
}

impl FromStr for RobotFactory {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let blueprints: Vec<Blueprint> = string
            .lines()
            .map(Blueprint::from_str)
            .collect::<Result<_, _>>()?;

        Ok(RobotFactory { blueprints })
    }
}

struct Blueprint {
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Blueprint {
    fn optimize_geodes(&self, time_limit: u32) -> u16 {
        const PRACTICALLY_UNLIMITED: u16 = u16::MAX;

        let max_spend_rates = Resources {
            ore: self
                .clay_robot_cost
                .ore
                .max(self.obsidian_robot_cost.ore)
                .max(self.geode_robot_cost.ore),
            clay: self.obsidian_robot_cost.clay,
            obsidian: self.geode_robot_cost.obsidian,
            geodes: PRACTICALLY_UNLIMITED,
        };

        let mut production_states = HashSet::from([ProductionState::default()]);

        for time in 0..time_limit {
            let time_remaining = (time_limit - time) as u16;

            let mut next_production_states = HashSet::new();

            for production_state in &production_states {
                if production_state.robots.ore != PRACTICALLY_UNLIMITED
                    && self.ore_robot_cost <= production_state.resources
                {
                    const ORE_ROBOT: Resources = Resources {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geodes: 0,
                    };

                    let mut next_state = ProductionState {
                        robots: production_state.robots + ORE_ROBOT,
                        resources: production_state.resources - self.ore_robot_cost
                            + production_state.robots,
                    };

                    // Coalesce all "more than we can possibly spend" states
                    if next_state.resources.ore + (next_state.robots.ore * (time_remaining - 1))
                        >= max_spend_rates.ore * time_remaining
                    {
                        next_state.robots.ore = PRACTICALLY_UNLIMITED;
                        next_state.resources.ore = PRACTICALLY_UNLIMITED;
                    }

                    next_production_states.insert(next_state);
                }

                if production_state.robots.clay != PRACTICALLY_UNLIMITED
                    && self.clay_robot_cost <= production_state.resources
                {
                    const CLAY_ROBOT: Resources = Resources {
                        ore: 0,
                        clay: 1,
                        obsidian: 0,
                        geodes: 0,
                    };

                    let mut next_state = ProductionState {
                        robots: production_state.robots + CLAY_ROBOT,
                        resources: production_state.resources - self.clay_robot_cost
                            + production_state.robots,
                    };

                    // Coalesce all "more than we can possibly spend" states
                    if next_state.resources.clay + (next_state.robots.clay * (time_remaining - 1))
                        >= max_spend_rates.clay * time_remaining
                    {
                        next_state.robots.clay = PRACTICALLY_UNLIMITED;
                        next_state.resources.clay = PRACTICALLY_UNLIMITED;
                    }

                    next_production_states.insert(next_state);
                }

                if production_state.robots.obsidian != PRACTICALLY_UNLIMITED
                    && self.obsidian_robot_cost <= production_state.resources
                {
                    const OBSIDIAN_ROBOT: Resources = Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 1,
                        geodes: 0,
                    };

                    let mut next_state = ProductionState {
                        robots: production_state.robots + OBSIDIAN_ROBOT,
                        resources: production_state.resources - self.obsidian_robot_cost
                            + production_state.robots,
                    };

                    // Coalesce all "more than we can possibly spend" states
                    if next_state.resources.obsidian
                        + (next_state.robots.obsidian * (time_remaining - 1))
                        >= max_spend_rates.obsidian * time_remaining
                    {
                        next_state.robots.obsidian = PRACTICALLY_UNLIMITED;
                        next_state.resources.obsidian = PRACTICALLY_UNLIMITED;
                    }

                    next_production_states.insert(next_state);
                }

                if self.geode_robot_cost <= production_state.resources {
                    const GEODE_ROBOT: Resources = Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geodes: 1,
                    };

                    next_production_states.insert(ProductionState {
                        robots: production_state.robots + GEODE_ROBOT,
                        resources: production_state.resources - self.geode_robot_cost
                            + production_state.robots,
                    });
                } else {
                    // Simply waiting and taking no action is always an option if we can't build a
                    // geode robot
                    next_production_states.insert(ProductionState {
                        robots: production_state.robots,
                        resources: production_state.resources + production_state.robots,
                    });
                }
            }

            production_states = next_production_states;
        }

        production_states
            .iter()
            .map(|production_state| production_state.resources.geodes)
            .max()
            .unwrap()
    }
}

impl FromStr for Blueprint {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BLUEPRINT_PATTERN: Regex = Regex::new(
                r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."
            )
            .unwrap();
        }

        if let Some(captures) = BLUEPRINT_PATTERN.captures(string) {
            Ok(Blueprint {
                ore_robot_cost: Resources {
                    ore: captures[1].parse()?,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
                clay_robot_cost: Resources {
                    ore: captures[2].parse()?,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
                obsidian_robot_cost: Resources {
                    ore: captures[3].parse()?,
                    clay: captures[4].parse()?,
                    obsidian: 0,
                    geodes: 0,
                },
                geode_robot_cost: Resources {
                    ore: captures[5].parse()?,
                    clay: 0,
                    obsidian: captures[6].parse()?,
                    geodes: 0,
                },
            })
        } else {
            Err("Could not parse blueprint".into())
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geodes == other.geodes
        {
            Some(Ordering::Equal)
        } else if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geodes <= other.geodes
        {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, addend: Self) -> Self::Output {
        Resources {
            ore: if self.ore == u16::MAX {
                u16::MAX
            } else {
                self.ore + addend.ore
            },

            clay: if self.clay == u16::MAX {
                u16::MAX
            } else {
                self.clay + addend.clay
            },

            obsidian: if self.obsidian == u16::MAX {
                u16::MAX
            } else {
                self.obsidian + addend.obsidian
            },

            geodes: if self.geodes == u16::MAX {
                u16::MAX
            } else {
                self.geodes + addend.geodes
            },
        }
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, subtrahend: Self) -> Self::Output {
        Resources {
            ore: if self.ore == u16::MAX {
                u16::MAX
            } else {
                self.ore - subtrahend.ore
            },

            clay: if self.clay == u16::MAX {
                u16::MAX
            } else {
                self.clay - subtrahend.clay
            },

            obsidian: if self.obsidian == u16::MAX {
                u16::MAX
            } else {
                self.obsidian - subtrahend.obsidian
            },

            geodes: if self.geodes == u16::MAX {
                u16::MAX
            } else {
                self.geodes - subtrahend.geodes
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct ProductionState {
    // This is a little gross, but for `robots`, `Resources` should be interpreted as "the number of
    // robots producing each kind of resource…"
    robots: Resources,

    // …while for `resources`, `Resources` should be interpreted as "the number of units of each
    // resource stored in inventory."
    resources: Resources,
}

impl Default for ProductionState {
    fn default() -> Self {
        Self {
            robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            },

            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_BLUEPRINTS: &str = indoc! {"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    fn test_optimize_geodes() {
        let blueprints: Vec<Blueprint> = TEST_BLUEPRINTS
            .lines()
            .map(Blueprint::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(9, blueprints[0].optimize_geodes(24));
        assert_eq!(12, blueprints[1].optimize_geodes(24));

        assert_eq!(56, blueprints[0].optimize_geodes(32));
        assert_eq!(62, blueprints[1].optimize_geodes(32));
    }

    #[test]
    fn test_quality_level_sum() {
        let factory = RobotFactory::from_str(TEST_BLUEPRINTS).unwrap();

        assert_eq!(33, factory.quality_level_sum());
    }
}
