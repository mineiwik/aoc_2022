use std::{collections::HashSet, num::ParseIntError};

struct Cave {
    rocks: HashSet<Position>,
    max_depth: isize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Cave {
    fn new() -> Self {
        Self {
            rocks: HashSet::new(),
            max_depth: 0,
        }
    }

    fn insert(&mut self, value: Position) {
        if value.y > self.max_depth {
            self.max_depth = value.y;
        }
        self.rocks.insert(value);
    }

    fn insert_and_update(&mut self, value: &mut Position, sum: &mut usize) {
        self.rocks.insert(*value);
        *value = Position::new(500, 0);
        *sum += 1;
    }

    fn contains(&self, value: &Position) -> bool {
        self.rocks.contains(value)
    }

    fn find_next(&self, from: &mut Position) -> bool {
        let mut tmp = *from;
        tmp.y += 1;
        if !self.contains(&tmp) {
            *from = tmp;
            return true;
        }
        tmp.x -= 1;
        if !self.contains(&tmp) {
            *from = tmp;
            return true;
        }
        tmp.x += 2;
        if !self.contains(&tmp) {
            *from = tmp;
            return true;
        }
        false
    }

    fn simulate_sand_with_void(&mut self) -> usize {
        let mut sum = 0;
        let mut sand = Position { x: 500, y: 0 };
        loop {
            if sand.y > self.max_depth {
                return sum;
            }
            if self.find_next(&mut sand) {
                continue;
            }
            self.insert_and_update(&mut sand, &mut sum)
        }
    }

    fn simulate_sand_with_ground(&mut self) -> usize {
        let mut sum = 0;
        let mut sand = Position { x: 500, y: 0 };
        loop {
            if sand.y == self.max_depth + 1 {
                self.insert_and_update(&mut sand, &mut sum);
                continue;
            }
            if self.find_next(&mut sand) {
                continue;
            }
            if sand.y == 0 {
                return sum + 1;
            }
            self.insert_and_update(&mut sand, &mut sum);
        }
    }
}

impl std::str::FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Self::new(
            parts.next().unwrap().parse()?,
            parts.next().unwrap().parse()?,
        ))
    }
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_group(group: &str, cave: &mut Cave) {
    let mut previous = None;
    for edge in group.split(" -> ") {
        let edge = edge.parse::<Position>().unwrap();
        cave.insert(edge);
        if let Some(mut previous) = previous {
            while previous != edge {
                cave.insert(previous);
                previous.x -= previous.x.cmp(&edge.x) as isize;
                previous.y -= previous.y.cmp(&edge.y) as isize;
            }
        }
        previous = Some(edge);
    }
}

fn parse_input(input: &str) -> Cave {
    let mut cave = Cave::new();
    for group in input.lines() {
        parse_group(group, &mut cave)
    }
    cave
}

fn part1(mut cave: Cave) -> usize {
    cave.simulate_sand_with_void()
}

fn part2(mut cave: Cave) -> usize {
    cave.simulate_sand_with_ground()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "24");
        assert_eq!(&part2, "93");
    }
}
