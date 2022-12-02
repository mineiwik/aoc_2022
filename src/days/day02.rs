use bevy::{prelude::*, utils::HashMap};
use std::fs;

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, _app: &mut App) {
        let stream: String = fs::read_to_string("assets/inputs/day02.txt").unwrap();
        println!("{:#?}", solve(&stream));
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Sign {
    ROCK,
    PAPER,
    SCISSORS,
}

pub fn solve(input: &str) -> (usize, usize) {
    let rounds: Vec<&str> = input.split("\n").collect();
    let mut strategies: Vec<(Sign, Sign)> = Vec::new();

    for round in rounds {
        let mut signs = round.split(" ");
        let opponent = signs.next().unwrap();
        let player = signs.next().unwrap();
        let opponent = match opponent {
            "A" => Sign::ROCK,
            "B" => Sign::PAPER,
            "C" => Sign::SCISSORS,
            _ => unimplemented!(),
        };
        let player = match player {
            "X" => Sign::ROCK,
            "Y" => Sign::PAPER,
            "Z" => Sign::SCISSORS,
            _ => unimplemented!(),
        };
        strategies.push((player, opponent));
    }

    let part1 = part1(&strategies);
    let part2 = part2(&strategies);

    (part1, part2)
}

fn part1(strategies: &Vec<(Sign, Sign)>) -> usize {
    let mut results = vec![];
    for strategy in strategies {
        let res = get_points(&strategy.0, &strategy.1);
        results.push(res);
    }
    results.iter().sum()
}

fn part2(strategies: &Vec<(Sign, Sign)>) -> usize {
    let mut results = vec![];
    for strategy in strategies {
        let player = get_player_sign(&strategy.1, &strategy.0);
        let res = get_points(&player, &strategy.1);
        results.push(res);
    }
    results.iter().sum()
}

fn get_points(player: &Sign, opponent: &Sign) -> usize {
    let mut sum = 0;
    sum += get_outcome_points(&player, &opponent);
    sum += match player {
        Sign::ROCK => 1,
        Sign::PAPER => 2,
        Sign::SCISSORS => 3,
    };
    sum
}

fn get_player_sign(opponent: &Sign, outcome: &Sign) -> Sign {
    match (opponent, outcome) {
        (Sign::ROCK, Sign::ROCK) => Sign::SCISSORS,
        (Sign::ROCK, Sign::SCISSORS) => Sign::PAPER,
        (Sign::PAPER, Sign::ROCK) => Sign::ROCK,
        (Sign::PAPER, Sign::SCISSORS) => Sign::SCISSORS,
        (Sign::SCISSORS, Sign::ROCK) => Sign::PAPER,
        (Sign::SCISSORS, Sign::SCISSORS) => Sign::ROCK,
        _ => opponent.clone(),
    }
}

fn get_outcome_points(player: &Sign, opponent: &Sign) -> usize {
    if *player == *opponent {
        return 3;
    }
    match (player, opponent) {
        (Sign::ROCK, Sign::SCISSORS)
        | (Sign::SCISSORS, Sign::PAPER)
        | (Sign::PAPER, Sign::ROCK) => {
            return 6;
        }
        _ => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "\
A Y
B X
C Z";
        let (part1, part2) = solve(input);

        assert_eq!(part1, 15);
        assert_eq!(part2, 12);
    }
}
