use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    const N: Position = Position { x: 0, y: -1 };
    const NE: Position = Position { x: 1, y: -1 };
    const E: Position = Position { x: 1, y: 0 };
    const SE: Position = Position { x: 1, y: 1 };
    const S: Position = Position { x: 0, y: 1 };
    const SW: Position = Position { x: -1, y: 1 };
    const W: Position = Position { x: -1, y: 0 };
    const NW: Position = Position { x: -1, y: -1 };

    const DIRECTIONS: [Position; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];

    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn add(&self, rhs: &Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> HashSet<Position> {
    let mut map = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, el) in row.chars().enumerate() {
            if el == '#' {
                map.insert(Position {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    map
}

fn simulate_round(map: &mut HashSet<Position>, i: isize) -> bool {
    let mut propositions: HashMap<Position, Position> = HashMap::new();
    for a in map.iter() {
        let mut free = vec![];
        for pos in Position::DIRECTIONS {
            if !map.contains(&a.add(&pos)) {
                free.push(a.add(&pos));
            }
        }
        if free.len() >= Position::DIRECTIONS.len() {
            continue;
        }
        for j in 0..4 {
            match (i + j) % 4 {
                0 => {
                    if !map.contains(&a.add(&Position::N))
                        && !map.contains(&a.add(&Position::NE))
                        && !map.contains(&a.add(&Position::NW))
                    {
                        propositions.insert(*a, a.add(&Position::N));
                        break;
                    }
                }
                1 => {
                    if !map.contains(&a.add(&Position::S))
                        && !map.contains(&a.add(&Position::SE))
                        && !map.contains(&a.add(&Position::SW))
                    {
                        propositions.insert(*a, a.add(&Position::S));
                        break;
                    }
                }
                2 => {
                    if !map.contains(&a.add(&Position::W))
                        && !map.contains(&a.add(&Position::NW))
                        && !map.contains(&a.add(&Position::SW))
                    {
                        propositions.insert(*a, a.add(&Position::W));
                        break;
                    }
                }
                3 => {
                    if !map.contains(&a.add(&Position::E))
                        && !map.contains(&a.add(&Position::NE))
                        && !map.contains(&a.add(&Position::SE))
                    {
                        propositions.insert(*a, a.add(&Position::E));
                        break;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    if propositions.len() == 0 {
        return true;
    }

    let vals: Vec<_> = propositions.values().collect();

    for (a, b) in &propositions {
        let mut i = 0;
        for val in &vals {
            if b == *val {
                i += 1;
            }
        }
        if i == 1 {
            map.remove(a);
            map.insert(*b);
        }
    }
    false
}

fn part1(mut map: HashSet<Position>) -> isize {
    for i in 0..10 {
        simulate_round(&mut map, i);
    }
    let mut min = Position::new(isize::MAX, isize::MAX);
    let mut max = Position::new(isize::MIN, isize::MIN);
    for a in &map {
        if a.x > max.x {
            max.x = a.x;
        }
        if a.x < min.x {
            min.x = a.x;
        }
        if a.y > max.y {
            max.y = a.y;
        }
        if a.y < min.y {
            min.y = a.y;
        }
    }
    let mut sum = 0;
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if !map.contains(&Position::new(x, y)) {
                sum += 1;
            }
        }
    }
    sum
}

fn part2(mut map: HashSet<Position>) -> isize {
    let mut sum = 0;
    loop {
        if simulate_round(&mut map, sum) {
            break;
        };
        sum += 1;
    }
    sum + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            ....#..
            ..###.#
            #...#.#
            .#...##
            #.###..
            ##.#.##
            .#..#..
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "110");
        assert_eq!(&part2, "20");
    }
}
