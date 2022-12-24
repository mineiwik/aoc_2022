use std::{collections::HashSet, hash::Hash};

const DIRECTIONS: [Position; 5] = [
    Position { x: 0, y: 0 },
    Position { x: 1, y: 0 },
    Position { x: -1, y: 0 },
    Position { x: 0, y: 1 },
    Position { x: 0, y: -1 },
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn add(&self, rhs: &Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug)]
struct Valley {
    blizzards: HashSet<(Position, Direction)>,
    max_x: isize,
    max_y: isize,
}

impl Valley {
    fn new(input: &str) -> Self {
        let mut blizzards = HashSet::new();
        for (y, row) in input.lines().enumerate() {
            for (x, el) in row.chars().enumerate() {
                let dir = match el {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => continue,
                };
                blizzards.insert((Position::new(x as isize - 1, y as isize - 1), dir));
            }
        }
        let rows: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        let max_x = rows.first().unwrap().len() as isize - 2;
        let max_y = rows.len() as isize - 2;
        Self {
            blizzards,
            max_x,
            max_y,
        }
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Valley {
    Valley::new(input)
}

fn part1(mut valley: Valley) -> isize {
    let start = Position::new(0, -1);
    let goal = Position::new(valley.max_x - 1, valley.max_y);
    min_time(&mut valley, &start, &goal)
}

fn part2(mut valley: Valley) -> isize {
    let mut sum = 0;
    let start = Position::new(0, -1);
    let goal = Position::new(valley.max_x - 1, valley.max_y);
    sum += min_time(&mut valley, &start, &goal);
    sum += min_time(&mut valley, &goal, &start);
    sum += min_time(&mut valley, &start, &goal);
    sum
}

fn min_time(valley: &mut Valley, start: &Position, goal: &Position) -> isize {
    let mut time = 0;
    let mut player_positions = HashSet::from([*start]);
    'outer: loop {
        for pos in &player_positions {
            if *pos == *goal {
                break 'outer time;
            }
        }

        let mut new_blizzards = HashSet::new();

        for (pos, dir) in &valley.blizzards {
            match dir {
                Direction::Right => new_blizzards.insert((
                    Position::new((pos.x + 1).rem_euclid(valley.max_x), pos.y),
                    *dir,
                )),
                Direction::Left => new_blizzards.insert((
                    Position::new((pos.x - 1).rem_euclid(valley.max_x), pos.y),
                    *dir,
                )),
                Direction::Up => new_blizzards.insert((
                    Position::new(pos.x, (pos.y - 1).rem_euclid(valley.max_y)),
                    *dir,
                )),
                Direction::Down => new_blizzards.insert((
                    Position::new(pos.x, (pos.y + 1).rem_euclid(valley.max_y)),
                    *dir,
                )),
            };
        }

        valley.blizzards = new_blizzards;

        let mut new_player_positions = HashSet::new();

        for pos in &player_positions {
            'inner: for dir in DIRECTIONS {
                let new_pos = pos.add(&dir);
                if new_pos.x < 0
                    || new_pos.x >= valley.max_x
                    || new_pos.y < 0
                    || new_pos.y >= valley.max_y
                {
                    if new_pos != Position::new(0, -1)
                        && new_pos != Position::new(valley.max_x - 1, valley.max_y)
                    {
                        continue;
                    }
                }
                for blizzard in &valley.blizzards {
                    if new_pos == blizzard.0 {
                        continue 'inner;
                    }
                }
                new_player_positions.insert(new_pos);
            }
        }

        player_positions = new_player_positions;

        time += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            #.######
            #>>.<^<#
            #.<..<<#
            #>v.><>#
            #<^v^^>#
            ######.#
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "18");
        assert_eq!(&part2, "54");
    }
}
