use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

use regex::Regex;

#[derive(Debug)]
struct Tunnel {
    nodes: HashMap<String, Node>,
}

impl std::str::FromStr for Tunnel {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([A-Z]{2})").unwrap();
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let valve = re
                .captures(line)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .to_owned();
            let node = line.parse()?;
            let node = nodes.insert(valve, node);
        }
        Ok(Self { nodes })
    }
}

#[derive(Debug)]
struct Node {
    value: isize,
    adjacencies: HashSet<String>,
}

impl std::str::FromStr for Node {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re_value = Regex::new(r"(\d+)").unwrap();
        let re_adjacencies = Regex::new(r"([A-Z]{2})(,\s[A-Z]{2})*$").unwrap();
        let value = re_value
            .captures(input)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<isize>()?;
        let adjacencies = re_adjacencies
            .captures(input)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|x| x.to_owned())
            .collect();
        Ok(Self { value, adjacencies })
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Tunnel {
    input.parse().unwrap()
}

fn most_pressure(
    current: &String,
    open_current: bool,
    remaining_time: isize,
    tunnel: &Tunnel,
    opened_valves: &mut Vec<String>,
    pressure: isize,
    dp: &mut HashMap<(String, Vec<String>, isize), (isize, isize, Vec<String>)>,
) -> isize {
    if remaining_time <= 0 {
        return 0;
    }

    let node = tunnel.nodes.get(current).unwrap();
    let mut additional = 0;

    if open_current {
        if opened_valves.contains(current) || node.value <= 0 {
            return 0;
        }
        opened_valves.push(current.clone());
        opened_valves.sort();
        additional = node.value * remaining_time;
    }

    if let Some(res) = dp.get(&(current.clone(), opened_valves.clone(), remaining_time)) {
        *opened_valves = res.2.clone();
        return res.0 + additional;
    }

    let mut best = isize::MIN;
    let mut best_opened_valves = opened_valves.clone();

    for adjacency in &node.adjacencies {
        for open_next in [true, false] {
            let mut next_opened_valves = opened_valves.clone();
            let next = most_pressure(
                adjacency,
                open_next,
                remaining_time - if open_next { 2 } else { 1 },
                tunnel,
                &mut next_opened_valves,
                pressure + additional,
                dp,
            );
            if next > best {
                best = next;
                best_opened_valves = next_opened_valves;
            }
        }
    }
    dp.insert(
        (current.clone(), opened_valves.clone(), remaining_time),
        (best, pressure + additional, best_opened_valves.clone()),
    );
    *opened_valves = best_opened_valves;
    additional + best
}

fn part1(tunnel: Tunnel) -> isize {
    most_pressure(
        &"AA".to_owned(),
        false,
        30,
        &tunnel,
        &mut vec![],
        0,
        &mut HashMap::new(),
    )
}

fn part2(tunnel: Tunnel) -> isize {
    let mut dp = HashMap::new();
    let mut sum = most_pressure(
        &"AA".to_owned(),
        false,
        26,
        &tunnel,
        &mut vec![],
        0,
        &mut dp,
    );

    for (k1, v1) in &dp {
        for (k2, v2) in &dp {
            if k1.1.len() == 0 || k2.1.len() == 0 {
                continue;
            }

            if v1.1 + v2.1 <= sum {
                continue;
            }

            let h1: HashSet<_> = k1.1.iter().collect();
            let h2: HashSet<_> = k2.1.iter().collect();
            if h1.is_disjoint(&h2) {
                if v1.1 + v2.1 > sum {
                    sum = v1.1 + v2.1;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
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
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "1651");
        assert_eq!(&part2, "1706");
    }
}
