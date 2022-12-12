use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    mem::swap,
};

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

type Position = (usize, usize);

#[derive(Clone)]
struct Heightmap {
    nodes: HashMap<Position, Node>,
    start: Position,
    end: Position,
}

#[derive(Clone)]
struct Node {
    value: char,
    edges: Vec<Position>,
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input, false));
    let part2 = part2(parse_input(input, true));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str, rev: bool) -> Heightmap {
    let input: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let h = input.len() as isize;
    let w = input.first().unwrap().len() as isize;

    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);

    let mut nodes: HashMap<Position, Node> = HashMap::new();

    for y in 0..h {
        for x in 0..w {
            let current = *input.get(y as usize).unwrap().get(x as usize).unwrap();
            if current == 'S' {
                start = (x as usize, y as usize);
            }
            if current == 'E' {
                end = (x as usize, y as usize);
            }
            let mut edges = vec![];
            for direction in DIRECTIONS {
                if is_edge(direction, x, y, w, h) {
                    continue;
                }
                let to: Position = ((direction.0 + x) as usize, (direction.1 + y) as usize);
                let neighbor = *input.get(to.1).unwrap().get(to.0).unwrap();
                if no_edge(current, neighbor, rev) {
                    continue;
                }
                edges.push(to);
            }
            nodes.insert(
                (x as usize, y as usize),
                Node {
                    value: current,
                    edges,
                },
            );
        }
    }
    Heightmap { nodes, start, end }
}

fn is_edge(direction: (isize, isize), x: isize, y: isize, w: isize, h: isize) -> bool {
    x + direction.0 < 0 || y + direction.1 < 0 || x + direction.0 >= w || y + direction.1 >= h
}

fn no_edge(current: char, neighbor: char, rev: bool) -> bool {
    let mut a = get_value(current);
    let mut b = get_value(neighbor);
    if rev {
        swap(&mut a, &mut b);
    }
    b as isize - a as isize > 1
}

fn get_value(input: char) -> char {
    if input == 'S' {
        return 'a';
    }
    if input == 'E' {
        return 'z';
    }
    input
}

fn part1(heightmap: Heightmap) -> usize {
    search(heightmap.nodes, heightmap.start, 'E')
}

fn part2(heightmap: Heightmap) -> usize {
    search(heightmap.nodes, heightmap.end, 'a')
}

fn search(graph: HashMap<Position, Node>, source: Position, destination: char) -> usize {
    let mut unvisited = VecDeque::new();
    let mut distance = 0;
    let mut visited = HashMap::new();
    unvisited.push_back(source);

    let mut current = (0, 0);

    while let Some(node) = unvisited.pop_front() {
        if graph.get(&node).unwrap().value == destination {
            current = node;
            break;
        }

        let edges = &graph.get(&node).unwrap().edges;

        for edge in edges {
            if let Entry::Vacant(e) = visited.entry(edge) {
                e.insert(node);
                unvisited.push_back(*edge);
            }
        }
    }

    while current != source {
        distance += 1;
        current = *visited.get(&current).unwrap();
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "31");
        assert_eq!(&part2, "29");
    }
}
