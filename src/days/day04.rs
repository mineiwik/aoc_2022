use std::collections::HashSet;

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let mut pairs: Vec<(HashSet<usize>, HashSet<usize>)> = vec![];

    for pair in input.lines() {
        let mut ranges = pair.split(",");
        let lhs = ranges.next().unwrap();
        let rhs = ranges.next().unwrap();

        let mut lhs = lhs.split("-").map(|x| x.parse::<usize>().unwrap());
        let lhs: HashSet<usize> = (lhs.next().unwrap()..=lhs.next().unwrap()).collect();

        let mut rhs = rhs.split("-").map(|x| x.parse::<usize>().unwrap());
        let rhs: HashSet<usize> = (rhs.next().unwrap()..=rhs.next().unwrap()).collect();

        pairs.push((lhs, rhs));
    }
    pairs
}

fn part1(pairs: Vec<(HashSet<usize>, HashSet<usize>)>) -> usize {
    let mut sum: usize = 0;
    for (lhs, rhs) in pairs {
        if lhs.is_subset(&rhs) || rhs.is_subset(&lhs) {
            sum += 1;
        }
    }
    sum
}

fn part2(pairs: Vec<(HashSet<usize>, HashSet<usize>)>) -> usize {
    let mut sum: usize = 0;
    for (lhs, rhs) in pairs {
        if !lhs.is_disjoint(&rhs) {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "2");
        assert_eq!(&part2, "4");
    }
}
