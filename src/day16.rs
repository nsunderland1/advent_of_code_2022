use std::collections::BTreeSet;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

#[allow(unused)]
use crate::prelude::*;

struct Valve<'a> {
    label: &'a str,
    rate: usize,
    tunnels: Vec<&'a str>,
}

fn parse_line(input: &str) -> IResult<&str, Valve<'_>> {
    map(
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            parse_usize,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
        )),
        |(_, label, _, rate, _, tunnels)| Valve {
            label,
            rate,
            tunnels,
        },
    )(input)
}

const TIME: usize = 30;
const TIME_2: usize = 26;

fn solve<'a, 'b>(
    current_valve: &'a str,
    time_left: usize,
    activated_set: &'b mut BTreeSet<&'a str>,
    valves: &HashMap<&'a str, Valve<'a>>,
    cache: &mut HashMap<(&'a str, usize), HashMap<BTreeSet<&'a str>, usize>>,
) -> usize {
    if time_left == 0 {
        return 0;
    }

    if let Some(cache_entry) = cache.get(&(current_valve, time_left)) {
        if let Some(&cache_entry) = cache_entry.get(activated_set) {
            return cache_entry;
        }
    }

    let current_valve = valves.get(current_valve).unwrap();

    let mut best = 0;

    if current_valve.rate > 0 && !activated_set.contains(current_valve.label) {
        activated_set.insert(current_valve.label);
        let time_left = time_left - 1;

        let subresult = solve(current_valve.label, time_left, activated_set, valves, cache);
        best = best.max(subresult + time_left * current_valve.rate);

        activated_set.remove(current_valve.label);
    }

    for neighbour in current_valve.tunnels.iter() {
        let time_left = time_left - 1;
        best = best.max(solve(neighbour, time_left, activated_set, valves, cache));
    }

    cache
        .entry((current_valve.label, time_left))
        .or_default()
        .insert(activated_set.clone(), best);

    best
}

pub fn run(input: &str) -> (Solution, Solution) {
    let valves: HashMap<_, _> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .map(|valve| (valve.label, valve))
        .collect();

    let mut cache = HashMap::default();

    let result1 = {
        let mut activated_set = BTreeSet::default();
        solve("AA", TIME, &mut activated_set, &valves, &mut cache)
    };

    let result2 = {
        let nonzero_valves: BTreeSet<&str> = valves
            .values()
            .filter(|valve| valve.rate > 0)
            .map(|valve| valve.label)
            .collect();

        nonzero_valves
            .iter()
            .copied()
            .powerset()
            .map(|subset| {
                let mut me: BTreeSet<&str> = subset.into_iter().collect();
                let mut elephant: BTreeSet<&str> =
                    nonzero_valves.difference(&me).copied().collect();

                solve("AA", TIME_2, &mut me, &valves, &mut cache)
                    + solve("AA", TIME_2, &mut elephant, &valves, &mut cache)
            })
            .max()
            .unwrap()
    };

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 16;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
