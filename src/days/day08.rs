const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|tree| tree.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn visibility_and_view_distance(
    forest: &Vec<Vec<usize>>,
    mut row: usize,
    mut col: usize,
    direction: (isize, isize),
) -> (bool, usize) {
    let h = forest.len();
    let w = forest.first().unwrap().len();
    let mut view_distance: usize = 0;
    let current_tree = *forest.get(row).unwrap().get(col).unwrap();
    while row > 0 && col > 0 && row < h - 1 && col < w - 1 {
        row = (row as isize + direction.0) as usize;
        col = (col as isize + direction.1) as usize;
        view_distance += 1;
        let next_tree = *forest.get(row).unwrap().get(col).unwrap();
        if current_tree <= next_tree {
            return (false, view_distance);
        }
    }
    (true, view_distance)
}

fn part1(forest: Vec<Vec<usize>>) -> usize {
    let h = forest.len();
    let w = forest.first().unwrap().len();
    let mut sum: usize = 0;

    for row in 0..h {
        for col in 0..w {
            let mut visible: bool = false;
            for direction in DIRECTIONS {
                visible |= visibility_and_view_distance(&forest, row, col, direction).0;
            }
            if visible {
                sum += 1;
            }
        }
    }
    sum
}

fn part2(forest: Vec<Vec<usize>>) -> usize {
    let h = forest.len();
    let w = forest.first().unwrap().len();
    let mut max: usize = 0;

    for row in 0..h {
        for col in 0..w {
            let mut scenic_score: usize = 1;
            for direction in DIRECTIONS {
                scenic_score *= visibility_and_view_distance(&forest, row, col, direction).1;
            }
            if scenic_score > max {
                max = scenic_score;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            30373
            25512
            65332
            33549
            35390
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "21");
        assert_eq!(&part2, "8");
    }
}
