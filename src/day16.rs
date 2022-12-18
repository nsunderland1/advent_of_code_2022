use std::cmp::Reverse;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

#[allow(unused)]
use crate::prelude::*;

#[derive(Debug)]
struct Valve<Label> {
    label: Label,
    rate: u8,
    tunnels: Vec<Label>,
}

struct Cache {
    store: Vec<Option<u16>>,
}

impl Cache {
    fn new() -> Self {
        let max_cache_entries = ((1 << 6) - 1) // valve label
        * ((1 << 5) - 1) // time remaining
        * ((1 << 15) - 1) // visited set
        * 4; // I'm gonna be honest, idk where this came from

        Self {
            store: vec![None; max_cache_entries],
        }
    }

    fn index(label: u8, time_left: u8, activated: u16) -> usize {
        ((activated as usize) << 11) | ((label as usize) << 5) | (time_left as usize)
    }

    fn get(&self, label: u8, time_left: u8, activated: u16) -> Option<u16> {
        self.store[Self::index(label, time_left, activated)]
    }

    fn insert(&mut self, label: u8, time_left: u8, activated: u16, value: u16) {
        self.store[Self::index(label, time_left, activated)] = Some(value);
    }
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Valve<&'a str>> {
    map(
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            nom::character::complete::u8,
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

const TIME: u8 = 30;
const TIME_2: u8 = 26;

fn solve(
    label: u8,
    time_left: u8,
    activated_set: u16,
    valves: &[Valve<u8>],
    cache: &mut Cache,
) -> u16 {
    if time_left == 0 {
        return 0;
    }

    if let Some(cache_entry) = cache.get(label, time_left, activated_set) {
        return cache_entry;
    }

    let current_valve = &valves[label as usize];

    let mut best = 0;

    if current_valve.rate > 0 && activated_set & (1 << label) == 0 {
        let new_set = activated_set | (1 << label);
        let time_left = time_left - 1;

        let subresult = solve(label, time_left, new_set, valves, cache);
        best = best.max(subresult + (time_left as u16) * (current_valve.rate as u16));
    }

    for &neighbour in current_valve.tunnels.iter() {
        best = best.max(solve(
            neighbour,
            time_left - 1,
            activated_set,
            valves,
            cache,
        ));
    }

    cache.insert(label, time_left, activated_set, best);

    best
}

pub fn run(input: &str) -> (Solution, Solution) {
    let valves_lookup: HashMap<&str, (u8, Valve<&str>)> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .sorted_by_key(|valve| Reverse(valve.rate))
        .enumerate()
        .map(|(index, valve)| (valve.label, (index as u8, valve)))
        .collect();

    let valves: Vec<Valve<u8>> = valves_lookup
        .iter()
        .map(|(_, (index, valve))| Valve {
            label: *index,
            rate: valve.rate,
            tunnels: valve
                .tunnels
                .iter()
                .map(|label| valves_lookup.get(label).unwrap().0)
                .collect(),
        })
        .sorted_by_key(|valve| valve.label)
        .collect();

    let aa = valves_lookup.get("AA").unwrap().0;

    let mut cache = Cache::new();

    let result1 = solve(aa, TIME, 0, &valves, &mut cache) as usize;

    let result2 = (0u16..((1 << 15) - 1))
        .map(|me| {
            let elephant = !me;

            solve(aa, TIME_2, me, &valves, &mut cache)
                + solve(aa, TIME_2, elephant, &valves, &mut cache)
        })
        .max()
        .unwrap() as usize;

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
