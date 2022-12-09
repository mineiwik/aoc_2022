use std::{cmp, collections::HashSet};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_along(&mut self, direction: char) {
        match direction {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'R' => self.x += 1,
            'L' => self.x -= 1,
            _ => unimplemented!(),
        }
    }

    fn follow(&mut self, head: &Self) {
        if self.is_in_range(head) {
            return;
        }
        self.x -= self.x.cmp(&head.x) as isize;
        self.y -= self.y.cmp(&head.y) as isize;
    }

    fn is_in_range(&self, rhs: &Self) -> bool {
        cmp::max((self.x - rhs.x).abs(), (self.y - rhs.y).abs()) <= 1
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .map(|x| {
            let mut parts = x.split(' ');
            (
                parts.next().unwrap().chars().next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn simulate_knots(instructions: Vec<(char, usize)>, amount: usize) -> usize {
    let mut knots: Vec<Position> = vec![Position::new(); amount];
    let mut positions: HashSet<Position> = HashSet::new();
    for (direction, amount) in instructions {
        for _ in 0..amount {
            knots.first_mut().unwrap().move_along(direction);
            let mut previous_knot = *knots.first().unwrap();
            for knot in knots.iter_mut().skip(1) {
                knot.follow(&previous_knot);
                previous_knot = *knot;
            }
            positions.insert(*knots.last().unwrap());
        }
    }
    positions.len()
}

fn part1(instructions: Vec<(char, usize)>) -> usize {
    simulate_knots(instructions, 2)
}

fn part2(instructions: Vec<(char, usize)>) -> usize {
    simulate_knots(instructions, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "13");
        assert_eq!(&part2, "1");
    }

    #[test]
    fn sample2() {
        let input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20        
        "};
        let (_, part2) = solve(input);

        assert_eq!(&part2, "36");
    }
}
