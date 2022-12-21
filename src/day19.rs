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

fn brute_force(
    blueprint: Blueprint,
    robots: Resources,
    time_left: usize,
    resources: Resources,
) -> usize {
    if time_left == 0 {
        return resources.geode;
    }

    let next_resources = Resources {
        ore: resources.ore + robots.ore,
        clay: resources.clay + robots.clay,
        obsidian: resources.obsidian + robots.obsidian,
        geode: resources.geode + robots.geode,
    };

    if resources.ore >= blueprint.geode.0 && resources.obsidian >= blueprint.geode.1 {
        return brute_force(
            blueprint,
            Resources {
                geode: robots.geode + 1,
                ..robots
            },
            time_left - 1,
            Resources {
                ore: next_resources.ore - blueprint.geode.0,
                obsidian: next_resources.obsidian - blueprint.geode.1,
                ..next_resources
            },
        );
    }

    if resources.ore >= blueprint.obsidian.0 && resources.clay >= blueprint.obsidian.1 {
        return brute_force(
            blueprint,
            Resources {
                obsidian: robots.obsidian + 1,
                ..robots
            },
            time_left - 1,
            Resources {
                ore: next_resources.ore - blueprint.obsidian.0,
                clay: next_resources.clay - blueprint.obsidian.1,
                ..next_resources
            },
        );
    }

    let mut best = 0;

    if resources.ore >= blueprint.clay {
        best = best.max(brute_force(
            blueprint,
            Resources {
                clay: robots.clay + 1,
                ..robots
            },
            time_left - 1,
            Resources {
                ore: next_resources.ore - blueprint.clay,
                ..next_resources
            },
        ));
    }

    if resources.ore >= blueprint.ore {
        best = best.max(brute_force(
            blueprint,
            Resources {
                ore: robots.ore + 1,
                ..robots
            },
            time_left - 1,
            Resources {
                ore: next_resources.ore - blueprint.ore,
                ..next_resources
            },
        ));
    }

    best = best.max(brute_force(
        blueprint,
        robots,
        time_left - 1,
        next_resources,
    ));

    best
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
                blueprint.id
                    * brute_force(
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
        // Part 2
        0
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
