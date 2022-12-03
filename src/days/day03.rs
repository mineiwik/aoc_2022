use bevy::{prelude::*, utils::HashSet};
use std::fs;

pub struct DayPlugin;


impl Plugin for DayPlugin {
    fn build(&self, _app: &mut App) {
        let stream: String = fs::read_to_string("assets/inputs/day03.txt").unwrap();
        println!("Day 3: {:?}", solve(&stream));
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1, part2)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(rucksacks: Vec<&str>) -> usize {
    let mut sum: usize = 0;
    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() >> 1);
        let first_compartment: HashSet<char> = first_compartment.chars().collect();
        let second_compartment: HashSet<char> = second_compartment.chars().collect();
        let intersection: Vec<&char> = first_compartment.intersection(&second_compartment).collect();
        sum += get_priority(intersection);
    }
    sum
}

fn part2(rucksacks: Vec<&str>) -> usize {
    let mut sum: usize = 0;
    for i in 0..rucksacks.len()/3 {
        let groups: Vec<HashSet<char>> = rucksacks[i*3..=i*3 + 2].iter().map(|group| group.chars().collect()).collect();
        let intersection: HashSet<char> = groups
        .iter()
        .skip(1)
        .fold(groups[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });
        let intersection = intersection.iter().collect();
        sum += get_priority(intersection);
    }
    sum
}

fn get_priority(intersection: Vec<&char>) -> usize {
    assert_eq!(intersection.len(), 1);
    let item = *intersection.first().unwrap();
    if item.is_lowercase() {
        return *item as usize - 96;
    }
    *item as usize - 38
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let (part1, part2) = solve(input);

        assert_eq!(part1, 157);
        assert_eq!(part2, 70);
    }
}
