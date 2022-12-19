use std::{collections::HashSet, num::ParseIntError};

use regex::Regex;

type Robots = (usize, usize, usize, usize);
type Storage = (usize, usize, usize, usize);

#[derive(Debug)]
struct Blueprint {
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
    max_ore_requirements: usize,
}

impl std::str::FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)").unwrap();
        let mut data = re.captures_iter(s).skip(1);
        let ore: usize = data.next().unwrap().get(0).unwrap().as_str().parse()?;
        let clay: usize = data.next().unwrap().get(0).unwrap().as_str().parse()?;
        let obsidian: (usize, usize) = (
            data.next().unwrap().get(0).unwrap().as_str().parse()?,
            data.next().unwrap().get(0).unwrap().as_str().parse()?,
        );
        let geode: (usize, usize) = (
            data.next().unwrap().get(0).unwrap().as_str().parse()?,
            data.next().unwrap().get(0).unwrap().as_str().parse()?,
        );
        let max_ore_requirements = ore.max(clay.max(obsidian.0.max(geode.0)));
        Ok(Self {
            ore,
            clay,
            obsidian,
            geode,
            max_ore_requirements,
        })
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    for line in input.lines() {
        blueprints.push(line.parse().unwrap());
    }
    blueprints
}

fn get_max_geode(
    blueprint: &Blueprint,
    storage: Storage,
    robots: Robots,
    time_remaining: usize,
    mut prohibit_build: (bool, bool, bool, bool),
    dp: &mut HashSet<(Storage, Robots, usize)>,
    max_geode: &mut usize,
) {
    if time_remaining == 0 {
        if storage.3 > *max_geode {
            *max_geode = storage.3;
        }
        return;
    }

    if dp.contains(&(storage, robots, time_remaining)) {
        return;
    }

    let mut max_estimate = storage.3;
    for i in robots.3..(robots.3 + time_remaining) {
        max_estimate += i;
    }

    if robots.0 >= blueprint.geode.0 && robots.2 >= blueprint.geode.1 {
        if max_estimate > *max_geode {
            *max_geode = max_estimate;
        }
        return;
    }

    if max_estimate < *max_geode {
        return;
    }

    let mut new_storage = storage;
    new_storage.0 += robots.0;
    new_storage.1 += robots.1;
    new_storage.2 += robots.2;
    new_storage.3 += robots.3;

    let mut robots_ready_to_build = 0;

    if !prohibit_build.0 && storage.0 >= blueprint.ore && robots.0 <= blueprint.max_ore_requirements
    {
        prohibit_build.0 = true;
        robots_ready_to_build += 1;
        let mut new_storage = new_storage;
        let mut new_robots = robots;
        new_storage.0 -= blueprint.ore;
        new_robots.0 += 1;
        get_max_geode(
            blueprint,
            new_storage,
            new_robots,
            time_remaining - 1,
            (false, false, false, false),
            dp,
            max_geode,
        );
    }

    if !prohibit_build.1 && storage.0 >= blueprint.clay && robots.1 <= blueprint.obsidian.1 {
        prohibit_build.1 = true;
        robots_ready_to_build += 1;
        let mut new_storage = new_storage;
        let mut new_robots = robots;
        new_storage.0 -= blueprint.clay;
        new_robots.1 += 1;
        get_max_geode(
            blueprint,
            new_storage,
            new_robots,
            time_remaining - 1,
            (false, false, false, false),
            dp,
            max_geode,
        );
    }

    if !prohibit_build.2
        && storage.0 >= blueprint.obsidian.0
        && storage.1 >= blueprint.obsidian.1
        && robots.2 <= blueprint.geode.1
    {
        prohibit_build.2 = true;
        robots_ready_to_build += 1;
        let mut new_storage = new_storage;
        let mut new_robots = robots;
        new_storage.0 -= blueprint.obsidian.0;
        new_storage.1 -= blueprint.obsidian.1;
        new_robots.2 += 1;
        get_max_geode(
            blueprint,
            new_storage,
            new_robots,
            time_remaining - 1,
            (false, false, false, false),
            dp,
            max_geode,
        );
    }

    if !prohibit_build.3 && storage.0 >= blueprint.geode.0 && storage.2 >= blueprint.geode.1 {
        prohibit_build.3 = true;
        robots_ready_to_build += 1;
        let mut new_storage = new_storage;
        let mut new_robots = robots;
        new_storage.0 -= blueprint.geode.0;
        new_storage.2 -= blueprint.geode.1;
        new_robots.3 += 1;
        get_max_geode(
            blueprint,
            new_storage,
            new_robots,
            time_remaining - 1,
            (false, false, false, false),
            dp,
            max_geode,
        );
    }

    if robots_ready_to_build < 4 {
        get_max_geode(
            blueprint,
            new_storage,
            robots,
            time_remaining - 1,
            prohibit_build,
            dp,
            max_geode,
        );
    }

    dp.insert((storage, robots, time_remaining));
}

fn part1(blueprints: Vec<Blueprint>) -> usize {
    let mut sum = 0;
    for (idx, blueprint) in blueprints.iter().enumerate() {
        let mut res = 0;
        get_max_geode(
            blueprint,
            (0, 0, 0, 0),
            (1, 0, 0, 0),
            24,
            (false, false, false, false),
            &mut HashSet::new(),
            &mut res,
        );
        sum += (idx + 1) * res;
    }
    sum
}

fn part2(blueprints: Vec<Blueprint>) -> usize {
    let mut prod = 1;
    for (idx, blueprint) in blueprints.iter().enumerate() {
        let mut res = 0;
        get_max_geode(
            blueprint,
            (0, 0, 0, 0),
            (1, 0, 0, 0),
            32,
            (false, false, false, false),
            &mut HashSet::new(),
            &mut res,
        );
        prod *= res;
        if idx == 2 {
            break;
        }
    }
    prod
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
            Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "33");
        assert_eq!(&part2, "3472");
    }
}
