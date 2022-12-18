use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

const SIDES: [Position; 6] = [
    Position { x: -1, y: 0, z: 0 },
    Position { x: 1, y: 0, z: 0 },
    Position { x: 0, y: -1, z: 0 },
    Position { x: 0, y: 1, z: 0 },
    Position { x: 0, y: 0, z: -1 },
    Position { x: 0, y: 0, z: 1 },
];

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn adjacent(&self, rhs: &Self) -> bool {
        let mut diff = 0;
        diff += (self.x - rhs.x).abs();
        diff += (self.y - rhs.y).abs();
        diff += (self.z - rhs.z).abs();
        diff == 1
    }

    fn add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    fn max(&self, rhs: &Self) -> Self {
        Self {
            x: std::cmp::max(self.x, rhs.x),
            y: std::cmp::max(self.y, rhs.y),
            z: std::cmp::max(self.z, rhs.z),
        }
    }

    fn min(&self, rhs: &Self) -> Self {
        Self {
            x: std::cmp::min(self.x, rhs.x),
            y: std::cmp::min(self.y, rhs.y),
            z: std::cmp::min(self.z, rhs.z),
        }
    }
}

struct World {
    cubes: HashSet<Position>,
    max: Position,
    min: Position,
}

impl World {
    fn new(cubes: HashSet<Position>) -> Self {
        let max = Position::new(i64::MIN, i64::MIN, i64::MIN);
        let min = Position::new(i64::MAX, i64::MAX, i64::MAX);
        Self { cubes, max, min }
    }

    fn contains(&self, cube: &Position) -> bool {
        self.cubes.contains(cube)
    }

    fn total_surface_area(&mut self) -> i64 {
        let mut sum = 0;
        for lhs_cube in &self.cubes {
            let mut current = 6;
            self.max = self.max.max(lhs_cube);
            self.min = self.min.min(lhs_cube);
            for rhs_cube in &self.cubes {
                if lhs_cube == rhs_cube {
                    continue;
                }
                if lhs_cube.adjacent(rhs_cube) {
                    current -= 1;
                }
                if current <= 0 {
                    break;
                }
            }
            sum += current;
        }
        sum
    }
}

impl std::str::FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse()?;
        let y = s.next().unwrap().parse()?;
        let z = s.next().unwrap().parse()?;
        Ok(Self { x, y, z })
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> HashSet<Position> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(cubes: HashSet<Position>) -> i64 {
    let mut world = World::new(cubes);
    world.total_surface_area()
}

fn part2(cubes: HashSet<Position>) -> i64 {
    let mut world = World::new(cubes);
    let mut sum = world.total_surface_area();
    world.max = world.max.add(&Position::new(1, 1, 1));
    world.min = world.min.add(&Position::new(-1, -1, -1));

    let mut empty_cubes = HashSet::new();

    for x in world.min.x..=world.max.x {
        for y in world.min.y..=world.max.y {
            for z in world.min.z..=world.max.z {
                let new_pos = Position::new(x, y, z);
                if !world.contains(&new_pos) {
                    empty_cubes.insert(new_pos);
                }
            }
        }
    }

    let mut adjacencies: HashMap<Position, Vec<Position>> = HashMap::new();

    for cube in &empty_cubes {
        let mut cube_adjacencies = vec![];
        for side in SIDES {
            let new_pos = cube.add(&side);
            if empty_cubes.contains(&new_pos) {
                cube_adjacencies.push(new_pos);
            }
        }
        adjacencies.insert(*cube, cube_adjacencies);
    }

    let mut accessable = HashSet::new();
    bfs(&world.min, &adjacencies, &mut accessable);

    for cube in empty_cubes {
        if !accessable.contains(&cube) {
            for occupied_cube in &world.cubes {
                if cube.adjacent(occupied_cube) {
                    sum -= 1;
                }
            }
        }
    }

    sum
}

fn bfs(
    current: &Position,
    adjacencies: &HashMap<Position, Vec<Position>>,
    accessable: &mut HashSet<Position>,
) {
    accessable.insert(*current);
    for adjacency in adjacencies.get(current).unwrap() {
        if !accessable.contains(adjacency) {
            bfs(adjacency, adjacencies, accessable);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            2,2,2
            1,2,2
            3,2,2
            2,1,2
            2,3,2
            2,2,1
            2,2,3
            2,2,4
            2,2,6
            1,2,5
            3,2,5
            2,1,5
            2,3,5
        "};
        let (part1, part2) = solve(input);
        assert_eq!(&part1, "64");
        assert_eq!(&part2, "58");
    }
}
