use std::collections::HashSet;

use bevy::utils::HashMap;

const SHAPES: [&str; 5] = ["####", ".#.|###|.#.", "###|..#|..#", "#|#|#|#", "##|##"];

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn add(&self, x: i64, y: i64) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}

#[derive(Debug)]
struct Shape {
    positions: HashSet<Position>,
}

impl Shape {
    fn new(shape: &str, origin: Position) -> Self {
        let mut positions = HashSet::new();
        for (y, line) in shape.split('|').enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                positions.insert(Position::new(origin.x + x as i64, origin.y + y as i64));
            }
        }
        Self { positions }
    }
}

#[derive(Debug)]
struct Playground {
    top: i64,
    left: i64,
    right: i64,
    map: HashSet<Position>,
}

impl Playground {
    fn new() -> Self {
        Self {
            top: 0,
            left: 0,
            right: 8,
            map: HashSet::new(),
        }
    }

    fn push(&self, dir: &char, shape: &mut Shape) {
        let dir: i64 = if *dir == '>' { 1 } else { -1 };
        let mut new_shape = Shape {
            positions: HashSet::new(),
        };
        for pos in &shape.positions {
            let new_pos = pos.add(dir, 0);
            if new_pos.x <= self.left || new_pos.x >= self.right {
                return;
            }
            if self.map.contains(&new_pos) {
                return;
            }
            new_shape.positions.insert(new_pos);
        }
        *shape = new_shape;
    }

    fn move_down(&self, shape: &mut Shape) -> bool {
        let mut new_shape = Shape {
            positions: HashSet::new(),
        };
        for pos in &shape.positions {
            let new_pos = pos.add(0, -1);
            if self.map.contains(&new_pos) || new_pos.y <= 0 {
                return true;
            }
            new_shape.positions.insert(new_pos);
        }
        *shape = new_shape;
        false
    }

    fn fix_shape(&mut self, shape: &Shape) {
        for pos in &shape.positions {
            if pos.y > self.top {
                self.top = pos.y;
            }
            self.map.insert(*pos);
        }
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<char> {
    input.lines().next().unwrap().chars().collect()
}

fn part1(directions: Vec<char>) -> i64 {
    let mut count_rocks = 0;
    let mut next_shape = SHAPES.iter().cycle();
    let mut playground = Playground::new();
    let mut shape = Shape::new(
        next_shape.next().unwrap(),
        Position::new(3, playground.top + 4),
    );
    for direction in directions.iter().cycle() {
        playground.push(direction, &mut shape);
        let settle_down = playground.move_down(&mut shape);
        if settle_down {
            playground.fix_shape(&shape);
            shape = Shape::new(
                next_shape.next().unwrap(),
                Position::new(3, playground.top + 4),
            );
            count_rocks += 1;
            if count_rocks >= 2022 {
                return playground.top;
            }
        }
    }
    0
}

fn part2(directions: Vec<char>) -> i64 {
    let mut count_rocks: i64 = 0;
    let mut next_shape = SHAPES.iter().cycle();
    let mut playground = Playground::new();
    let mut dp: HashMap<(usize, &str, usize), (i64, i64)> = HashMap::new();
    let mut current_shape = next_shape.next().unwrap();
    let mut shape = Shape::new(current_shape, Position::new(3, playground.top + 4));
    let mut cycle_detected = false;
    let mut upper = 0;
    let mut max_height = 0;
    let mut height = 0;
    for (idx, direction) in directions.iter().cycle().enumerate() {
        playground.push(direction, &mut shape);
        if playground.move_down(&mut shape) {
            if height > max_height {
                max_height = height;
            }
            height = 0;
            playground.fix_shape(&shape);
            current_shape = next_shape.next().unwrap();
            shape = Shape::new(current_shape, Position::new(3, playground.top + 4));
            count_rocks += 1;
            if count_rocks >= 1000000000000 {
                return playground.top + upper;
            }
            if !cycle_detected {
                if let Some(a) = dp.get(&(idx % directions.len(), current_shape, max_height)) {
                    let height_per_cycle = playground.top - a.1;
                    let rocks_per_cycle = count_rocks - a.0;
                    let amount_of_cycles = 1000000000000 / rocks_per_cycle - 2;
                    cycle_detected = true;
                    upper = amount_of_cycles * height_per_cycle;
                    count_rocks += amount_of_cycles * rocks_per_cycle;
                }
                dp.insert(
                    (idx % directions.len(), current_shape, max_height),
                    (count_rocks, playground.top),
                );
            }
        }
        height += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let (part1, part2) = solve(input);
        assert_eq!(&part1, "3068");
        assert_eq!(&part2, "1514285714288");
    }
}
