use bevy::prelude::*;
use std::fs;

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, _app: &mut App) {
        solve();
    }
}

pub fn solve() {
    let stream: String = fs::read_to_string("assets/inputs/day01.txt").unwrap();
    let elves: Vec<&str> = stream.split("\n\n").collect();

    let mut cals: Vec<usize> = Vec::new();

    for elve in elves {
        let sum: usize = elve.split("\n").map(|i| i.parse::<usize>().unwrap()).sum();
        cals.push(sum);
    }

    println!("{}", cals.iter().max().unwrap());

    cals.sort_by(|a, b| b.cmp(a));

    let sum = cals.get(0).unwrap() + cals.get(1).unwrap() + cals.get(2).unwrap();
    println!("{:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
}
