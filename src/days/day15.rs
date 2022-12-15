use std::{cmp, collections::HashSet, num::ParseIntError};

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn get_distance(&self, beacon: &Self) -> i64 {
        (self.x - beacon.x).abs() + (self.y - beacon.y).abs()
    }
}

impl std::str::FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", y=");
        let x = parts.next().unwrap().parse()?;
        let y = parts.next().unwrap().parse()?;
        Ok(Position::new(x, y))
    }
}

pub fn solve(input: &str, half_grid: i64) -> (String, String) {
    let part1 = part1(parse_input(input), half_grid);
    let part2 = part2(parse_input(input), 2 * half_grid);

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<(Position, Position)> {
    let mut res = vec![];
    let re = Regex::new(r"(-?\d+, .=-?\d+)").unwrap();
    for line in input.lines() {
        let mut positions = re.find_iter(line);
        let scanner: Position = positions.next().unwrap().as_str().parse().unwrap();
        let beacon: Position = positions.next().unwrap().as_str().parse().unwrap();
        res.push((scanner, beacon));
    }
    res
}

fn part1(input: Vec<(Position, Position)>, half_grid: i64) -> usize {
    let mut occupied: HashSet<i64> = HashSet::new();
    let mut half_grid_beacons = HashSet::new();
    for (scanner, beacon) in &input {
        if beacon.y == half_grid {
            half_grid_beacons.insert(beacon);
        }
        let distance = scanner.get_distance(beacon);
        let lower_bound = scanner.y - distance;
        let higher_bound = scanner.y + distance;
        if lower_bound <= half_grid && half_grid <= higher_bound {
            let half_width = ((half_grid - scanner.y).abs() - distance).abs();
            for x in -half_width..=half_width {
                occupied.insert(scanner.x + x);
            }
        }
    }
    occupied.len() - half_grid_beacons.len()
}

fn part2(input: Vec<(Position, Position)>, full_grid: i64) -> i64 {
    for (scanner, beacon) in &input {
        let distance = scanner.get_distance(beacon) + 1;
        let lower_bound = cmp::max(0, scanner.x - distance);
        let higher_bound = cmp::min(full_grid, scanner.x + distance);
        for x in lower_bound..=higher_bound {
            'outer: for j in [-1, 1] {
                let edge = Position::new(x, scanner.y + j * (distance - (x - scanner.x).abs()));
                if edge.x >= 0 && edge.x <= full_grid && edge.y >= 0 && edge.y <= full_grid {
                    for (other_scanner, other_beacon) in &input {
                        if other_scanner.get_distance(&edge)
                            <= other_scanner.get_distance(other_beacon)
                        {
                            continue 'outer;
                        }
                    }
                    return edge.x * 4000000 + edge.y;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "};
        let (part1, part2) = solve(input, 10);

        assert_eq!(&part1, "26");
        assert_eq!(&part2, "56000011");
    }
}
