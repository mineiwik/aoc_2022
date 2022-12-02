use bevy::prelude::*;
use std::fs;

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, _app: &mut App) {
        let stream: String = fs::read_to_string("assets/inputs/day01.txt").unwrap();
        println!("Day 1: {:?}", solve(&stream));
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1, part2)
}

fn parse_input(input: &str) -> Vec<usize> {
    let input = input.lines().collect::<Vec<&str>>().join("|");
    let elves: Vec<&str> = input.split("||").collect();
    let mut calories: Vec<usize> = Vec::new();
    for elve in elves {
        let sum: usize = elve.split("|").map(|i| i.parse::<usize>().unwrap()).sum();
        calories.push(sum);
    }
    calories
}

fn part1(calories: Vec<usize>) -> usize {
    *calories.iter().max().unwrap()
}

fn part2(mut calories: Vec<usize>) -> usize {
    calories.sort_by(|a, b| b.cmp(a));
    calories[0..=2].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let (part1, part2) = solve(input);

        assert_eq!(part1, 24000);
        assert_eq!(part2, 45000);
    }
}
