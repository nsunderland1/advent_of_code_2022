use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

#[allow(unused)]
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

impl Blueprint {
    fn most_ore_needed(&self) -> usize {
        [self.ore, self.clay, self.obsidian.0, self.geode.0]
            .into_iter()
            .max()
            .unwrap()
    }

    fn most_clay_needed(&self) -> usize {
        self.obsidian.1
    }

    fn most_obsidian_needed(&self) -> usize {
        self.geode.1
    }
}

fn parse_line(input: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            tag("Blueprint "),
            parse_usize,
            tag(": Each ore robot costs "),
            parse_usize,
            tag(" ore. Each clay robot costs "),
            parse_usize,
            tag(" ore. Each obsidian robot costs "),
            parse_usize,
            tag(" ore and "),
            parse_usize,
            tag(" clay. Each geode robot costs "),
            parse_usize,
            tag(" ore and "),
            parse_usize,
            tag(" obsidian."),
        )),
        |(
            _,
            id,
            _,
            ore,
            _,
            clay,
            _,
            obsidian_ore,
            _,
            obsidian_clay,
            _,
            geode_ore,
            _,
            geode_obsidian,
            _,
        )| {
            Blueprint {
                id,
                ore,
                clay,
                obsidian: (obsidian_ore, obsidian_clay),
                geode: (geode_ore, geode_obsidian),
            }
        },
    )(input)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

trait CheckedDivCeil: Sized {
    fn checked_div_ceil(self, other: Self) -> Option<Self>;
}

impl CheckedDivCeil for usize {
    fn checked_div_ceil(self, other: Self) -> Option<Self> {
        let remainder = self.checked_rem(other)?;
        let quotient = self.checked_div(other)?;

        Some(if remainder != 0 {
            quotient + 1
        } else {
            quotient
        })
    }
}

fn brute_force(
    mut best: usize,
    cache: &mut HashMap<(Resources, usize, Resources), usize>,
    blueprint: Blueprint,
    robots: Resources,
    time_left: usize,
    resources: Resources,
) -> usize {
    if time_left == 0 {
        return resources.geode;
    }

    if let Some(entry) = cache.get(&(robots, time_left, resources)) {
        return *entry;
    }

    if best
        >= robots.geode * time_left + resources.geode + time_left * time_left.saturating_sub(1) / 2
    {
        return 0;
    }

    let mut inner = || {
        let next_resources = |time_delta| Resources {
            ore: resources.ore + robots.ore * time_delta,
            clay: resources.clay + robots.clay * time_delta,
            obsidian: resources.obsidian + robots.obsidian * time_delta,
            geode: resources.geode + robots.geode * time_delta,
        };

        {
            let ore_needed = blueprint.geode.0.saturating_sub(resources.ore);
            let obsidian_needed = blueprint.geode.1.saturating_sub(resources.obsidian);

            let time_to_ore = ore_needed
                .checked_div_ceil(robots.ore)
                .unwrap_or(usize::MAX);
            let time_to_obsidian = obsidian_needed
                .checked_div_ceil(robots.obsidian)
                .unwrap_or(usize::MAX);
            let time_needed = usize::max(time_to_ore, time_to_obsidian);
            let time_needed = time_needed.saturating_add(1);

            if time_left >= time_needed {
                let next_resources = next_resources(time_needed);

                best = best.max(brute_force(
                    best,
                    cache,
                    blueprint,
                    Resources {
                        geode: robots.geode + 1,
                        ..robots
                    },
                    time_left - time_needed,
                    Resources {
                        ore: next_resources.ore - blueprint.geode.0,
                        obsidian: next_resources.obsidian - blueprint.geode.1,
                        ..next_resources
                    },
                ));
            }
        }

        if robots.obsidian < blueprint.most_obsidian_needed() {
            let ore_needed = blueprint.obsidian.0.saturating_sub(resources.ore);
            let clay_needed = blueprint.obsidian.1.saturating_sub(resources.clay);

            let time_to_ore = ore_needed
                .checked_div_ceil(robots.ore)
                .unwrap_or(usize::MAX);
            let time_to_clay = clay_needed
                .checked_div_ceil(robots.clay)
                .unwrap_or(usize::MAX);
            let time_needed = usize::max(time_to_ore, time_to_clay);
            let time_needed = time_needed.saturating_add(1);

            if time_left >= time_needed {
                let next_resources = next_resources(time_needed);

                best = best.max(brute_force(
                    best,
                    cache,
                    blueprint,
                    Resources {
                        obsidian: robots.obsidian + 1,
                        ..robots
                    },
                    time_left - time_needed,
                    Resources {
                        ore: next_resources.ore - blueprint.obsidian.0,
                        clay: next_resources.clay - blueprint.obsidian.1,
                        ..next_resources
                    },
                ));
            }
        }

        if robots.clay < blueprint.most_clay_needed() {
            let ore_needed = blueprint.clay.saturating_sub(resources.ore);
            let time_needed = ore_needed
                .checked_div_ceil(robots.ore)
                .unwrap_or(usize::MAX);
            let time_needed = time_needed.saturating_add(1);

            if time_left >= time_needed {
                let next_resources = next_resources(time_needed);

                best = best.max(brute_force(
                    best,
                    cache,
                    blueprint,
                    Resources {
                        clay: robots.clay + 1,
                        ..robots
                    },
                    time_left - time_needed,
                    Resources {
                        ore: next_resources.ore - blueprint.clay,
                        ..next_resources
                    },
                ));
            }
        }

        if robots.ore < blueprint.most_ore_needed() {
            let ore_needed = blueprint.ore.saturating_sub(resources.ore);
            let time_needed = ore_needed
                .checked_div_ceil(robots.ore)
                .unwrap_or(usize::MAX);
            let time_needed = time_needed.saturating_add(1);

            if time_left >= time_needed {
                let next_resources = next_resources(time_needed);

                best = best.max(brute_force(
                    best,
                    cache,
                    blueprint,
                    Resources {
                        ore: robots.ore + 1,
                        ..robots
                    },
                    time_left - time_needed,
                    Resources {
                        ore: next_resources.ore - blueprint.ore,
                        ..next_resources
                    },
                ));
            }
        }

        best
    };

    let result = inner();
    cache.insert((robots, time_left, resources), result);
    return result;
}

pub fn run(input: &str) -> (Solution, Solution) {
    let blueprints: Vec<_> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let result1 = {
        blueprints
            .iter()
            .copied()
            .map(|blueprint| {
                let mut cache = HashMap::default();
                blueprint.id
                    * brute_force(
                        0,
                        &mut cache,
                        blueprint,
                        Resources {
                            ore: 1,
                            clay: 0,
                            obsidian: 0,
                            geode: 0,
                        },
                        24,
                        Resources {
                            ore: 0,
                            clay: 0,
                            obsidian: 0,
                            geode: 0,
                        },
                    )
            })
            .sum::<usize>()
    };

    let result2 = {
        blueprints
            .iter()
            .take(3)
            .copied()
            .map(|blueprint| {
                let mut cache = HashMap::default();
                brute_force(
                    0,
                    &mut cache,
                    blueprint,
                    Resources {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    32,
                    Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                )
            })
            .product::<usize>()
    };

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 19;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
