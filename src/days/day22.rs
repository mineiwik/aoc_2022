use num_derive::{self, FromPrimitive};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
struct AoCParsingError;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
enum Rotation {
    CW,
    CCW,
}

impl FromStr for Rotation {
    type Err = AoCParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::CW),
            "L" => Ok(Self::CCW),
            _ => Err(AoCParsingError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Number(usize),
    Rotation(Rotation),
}

#[derive(Debug)]
struct Playground {
    map: HashMap<Position, Tile>,
    inst: Vec<Instruction>,
    player: Player,
}

impl Playground {
    fn execute(&mut self) {
        for inst in self.inst.clone() {
            match inst {
                Instruction::Number(val) => self.move_player(val),
                Instruction::Rotation(rot) => self.rotate_player(rot),
            }
        }
    }

    fn execute_new(&mut self, cube: &HashMap<(usize, usize), (Side, usize)>, size: usize) {
        for inst in self.inst.clone() {
            match inst {
                Instruction::Number(val) => self.move_player_new(val, cube, size),
                Instruction::Rotation(rot) => self.rotate_player(rot),
            }
        }
    }

    fn move_player(&mut self, val: usize) {
        for _ in 0..val {
            let mut new_pos = Position::new(self.player.pos.x, self.player.pos.y);
            match self.player.dir {
                Direction::Right => new_pos.x += 1,
                Direction::Down => new_pos.y += 1,
                Direction::Left => new_pos.x -= 1,
                Direction::Up => new_pos.y -= 1,
            }
            if self.map.get(&new_pos).is_none() {
                match self.player.dir {
                    Direction::Right => new_pos.x = self.get_min_x_for_y(new_pos.y),
                    Direction::Down => new_pos.y = self.get_min_y_for_x(new_pos.x),
                    Direction::Left => new_pos.x = self.get_max_x_for_y(new_pos.y),
                    Direction::Up => new_pos.y = self.get_max_y_for_x(new_pos.x),
                }
            };
            if let Some(tile) = self.map.get(&new_pos) {
                match tile {
                    Tile::Open => self.player.pos = new_pos,
                    Tile::Wall => return,
                }
                continue;
            }
        }
    }

    fn move_player_new(
        &mut self,
        val: usize,
        cube: &HashMap<(usize, usize), (Side, usize)>,
        size: usize,
    ) {
        for _ in 0..val {
            let mut new_pos = Position::new(self.player.pos.x, self.player.pos.y);
            let mut new_dir = None;
            match self.player.dir {
                Direction::Right => new_pos.x += 1,
                Direction::Down => new_pos.y += 1,
                Direction::Left => new_pos.x -= 1,
                Direction::Up => new_pos.y -= 1,
            }
            if self.map.get(&new_pos).is_none() {
                let (side, rot) = cube
                    .get(&(
                        (self.player.pos.x - 1) / size + 1,
                        (self.player.pos.y - 1) / size + 1,
                    ))
                    .unwrap();
                let other = CUBE_SIDE_LINKS[*side as usize][(self.player.dir as usize + rot) % 4];
                let other_face = cube.iter().find(|x| x.1 .0 == other).map(|x| x.0).unwrap();
                let (_, rot2) = cube.get(other_face).unwrap();
                let other_dir = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                ]
                .iter()
                .find(|orientation| {
                    CUBE_SIDE_LINKS[other as usize][(**orientation as usize + rot2).rem_euclid(4)]
                        == *side
                })
                .unwrap();

                let current_pos = Position::new(
                    (self.player.pos.x - 1) % size,
                    (self.player.pos.y - 1) % size,
                );
                let (n_x, n_y) = match (self.player.dir, other_dir) {
                    (Direction::Down, Direction::Down) => (size - current_pos.x, size),
                    (Direction::Down, Direction::Up) => (current_pos.x + 1, 1),
                    (Direction::Down, Direction::Left) => (1, size - current_pos.x),
                    (Direction::Down, Direction::Right) => (size, current_pos.x + 1),
                    (Direction::Up, Direction::Down) => (current_pos.x + 1, size),
                    (Direction::Up, Direction::Up) => (size - current_pos.x, 1),
                    (Direction::Up, Direction::Left) => (1, current_pos.x + 1),
                    (Direction::Up, Direction::Right) => (size, size - current_pos.x),
                    (Direction::Left, Direction::Down) => (size - current_pos.y, size),
                    (Direction::Left, Direction::Up) => (current_pos.y + 1, 1),
                    (Direction::Left, Direction::Left) => (1, size - current_pos.y),
                    (Direction::Left, Direction::Right) => (size, current_pos.y + 1),
                    (Direction::Right, Direction::Down) => (current_pos.y + 1, size),
                    (Direction::Right, Direction::Up) => (size - current_pos.y, 1),
                    (Direction::Right, Direction::Left) => (1, current_pos.y + 1),
                    (Direction::Right, Direction::Right) => (size, size - current_pos.y),
                };
                new_dir = Some(other_dir.clone().rotate(Rotation::CW).rotate(Rotation::CW));
                new_pos.x = n_x + size * (other_face.0 - 1);
                new_pos.y = n_y + size * (other_face.1 - 1);
            };
            if let Some(tile) = self.map.get(&new_pos) {
                match tile {
                    Tile::Open => {
                        self.player.pos = new_pos;
                        if let Some(new_dir) = new_dir {
                            self.player.dir = new_dir;
                        }
                    }
                    Tile::Wall => return,
                }
                continue;
            }
        }
    }

    fn get_min_y_for_x(&self, x: usize) -> usize {
        let mut min_y = usize::MAX;
        for pos in self.map.keys() {
            if pos.x == x && pos.y < min_y {
                min_y = pos.y;
            }
        }
        min_y
    }

    fn get_min_x_for_y(&self, y: usize) -> usize {
        let mut min_x = usize::MAX;
        for pos in self.map.keys() {
            if pos.y == y && pos.x < min_x {
                min_x = pos.x;
            }
        }
        min_x
    }

    fn get_max_y_for_x(&self, x: usize) -> usize {
        let mut max_y = usize::MIN;
        for pos in self.map.keys() {
            if pos.x == x && pos.y > max_y {
                max_y = pos.y;
            }
        }
        max_y
    }

    fn get_max_x_for_y(&self, y: usize) -> usize {
        let mut max_x = usize::MIN;
        for pos in self.map.keys() {
            if pos.y == y && pos.x > max_x {
                max_x = pos.x;
            }
        }
        max_x
    }

    fn rotate_player(&mut self, rot: Rotation) {
        self.player.dir.rotate(rot);
    }
}

impl FromStr for Playground {
    type Err = AoCParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_data, inst) = s.rsplit_once("\n\n").unwrap_or(("", ""));
        let re_inst = Regex::new(r"(\d+|[LR])").or(Err(AoCParsingError))?;
        let inst: Vec<Instruction> = re_inst
            .find_iter(inst)
            .map(|x| x.as_str().parse())
            .collect::<Result<Vec<_>, _>>()?;

        let mut map = HashMap::new();
        let mut min_y = usize::MAX;
        let mut min_x = usize::MAX;

        for (y, row) in map_data.lines().enumerate() {
            for (x, el) in row.chars().enumerate() {
                let x = x + 1;
                let y = y + 1;
                match el {
                    '.' => map.insert(Position::new(x, y), Tile::Open),
                    '#' => map.insert(Position::new(x, y), Tile::Wall),
                    ' ' => None,
                    _ => return Err(AoCParsingError),
                };
                if el != ' ' && y < min_y {
                    min_y = y;
                }
            }
        }

        for pos in map.keys() {
            if pos.y == min_y && pos.x < min_x {
                min_x = pos.x;
            }
        }

        let player = Player {
            pos: Position::new(min_x, min_y),
            dir: Direction::Right,
        };
        Ok(Self { map, inst, player })
    }
}

#[derive(Debug)]
struct Player {
    pos: Position,
    dir: Direction,
}

#[derive(Debug, FromPrimitive, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    const DIRECTIONS: [Self; 4] = [
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Up,
    ];
    fn rotate(&mut self, rot: Rotation) -> Self {
        let val = *self as isize;
        match rot {
            Rotation::CW => {
                *self = num::FromPrimitive::from_usize((val + 1).rem_euclid(4) as usize).unwrap()
            }
            Rotation::CCW => {
                *self = num::FromPrimitive::from_usize((val - 1).rem_euclid(4) as usize).unwrap()
            }
        }
        *self
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
    Open,
}

impl FromStr for Instruction {
    type Err = AoCParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(s) = s.parse() {
            return Ok(Self::Number(s));
        }
        Ok(Self::Rotation(s.parse()?))
    }
}

pub fn solve(input: &str, size: usize) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input), input, size);

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Playground {
    input.parse().unwrap()
}

fn part1(mut playground: Playground) -> usize {
    playground.execute();
    1000 * playground.player.pos.y + 4 * playground.player.pos.x + playground.player.dir as usize
}

fn part2(mut playground: Playground, s: &str, size: usize) -> usize {
    let (map_data, _) = s.rsplit_once("\n\n").unwrap_or(("", ""));
    let mut max_y = 0;
    let mut max_x = 0;

    let mut map = HashMap::new();

    for (y, row) in map_data.lines().enumerate() {
        for (x, el) in row.chars().enumerate() {
            let x = x + 1;
            let y = y + 1;
            match el {
                '.' => map.insert(Position::new(x, y), Tile::Open),
                '#' => map.insert(Position::new(x, y), Tile::Wall),
                _ => None,
            };
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    let mut faces: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..max_y / size {
        for x in 0..max_x / size {
            if map.contains_key(&Position::new(x * size + 1, y * size + 1)) {
                faces.insert((x + 1, y + 1));
            }
        }
    }

    let mut cube: HashMap<(usize, usize), (Side, usize)> = HashMap::new();

    fold_cube(
        Side::Bottom,
        0,
        &mut cube,
        &faces,
        *faces.iter().next().unwrap(),
    );

    playground.execute_new(&cube, size);

    1000 * playground.player.pos.y + 4 * playground.player.pos.x + playground.player.dir as usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    Bottom = 0,
    Right = 1,
    Back = 2,
    Left = 3,
    Front = 4,
    Top = 5,
}

const CUBE_SIDE_LINKS: [[Side; 4]; 6] = [
    [Side::Right, Side::Back, Side::Left, Side::Front],
    [Side::Top, Side::Back, Side::Bottom, Side::Front],
    [Side::Right, Side::Top, Side::Left, Side::Bottom],
    [Side::Bottom, Side::Back, Side::Top, Side::Front],
    [Side::Right, Side::Bottom, Side::Left, Side::Top],
    [Side::Right, Side::Front, Side::Left, Side::Back],
];

fn fold_cube(
    current: Side,
    rot: usize,
    cube: &mut HashMap<(usize, usize), (Side, usize)>,
    faces: &HashSet<(usize, usize)>,
    c_face: (usize, usize),
) {
    cube.insert(c_face, (current, rot));

    for dir in Direction::DIRECTIONS {
        let (x, y) = match dir {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        };
        let new_c_face = (
            (c_face.0 as isize + x) as usize,
            (c_face.1 as isize + y) as usize,
        );
        if faces.contains(&new_c_face) && !cube.contains_key(&new_c_face) {
            let next = CUBE_SIDE_LINKS[current as usize][(dir as usize + rot) % 4];
            let rot = (0..4)
                .find(|orientation| {
                    CUBE_SIDE_LINKS[next as usize][(dir as usize + 2 + orientation) % 4] == current
                })
                .unwrap();
            fold_cube(next, rot, cube, faces, new_c_face);
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
                    ...#
                    .#..
                    #...
                    ....
            ...#.......#
            ........#...
            ..#....#....
            ..........#.
                    ...#....
                    .....#..
                    .#......
                    ......#.

            10R5L5R10L4R5L5
        "};
        let (part1, part2) = solve(input, 4);

        assert_eq!(&part1, "6032");
        assert_eq!(&part2, "5031");
    }
}
