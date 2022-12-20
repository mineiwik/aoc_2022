const POSITIONS: [usize; 3] = [1000, 2000, 3000];
const DECRYPTION_KEY: i64 = 811589153;

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .enumerate()
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect()
}

fn do_mixing(file: &mut Vec<(usize, i64)>) {
    let mut idx = 0;
    while idx < file.len() {
        let acutal_idx = file
            .iter()
            .enumerate()
            .find(|(_, x)| x.0 == idx)
            .map(|x| x.0)
            .unwrap() as i64;
        let el = file.remove(acutal_idx as usize);
        file.insert(
            (el.1 + acutal_idx).rem_euclid(file.len() as i64) as usize,
            el,
        );
        idx += 1;
    }
}

fn get_coordinate_sum(file: &Vec<(usize, i64)>) -> i64 {
    let start_idx = file
        .iter()
        .enumerate()
        .find(|(_, x)| x.1 == 0)
        .map(|x| x.0)
        .unwrap();
    POSITIONS
        .iter()
        .map(|i| file.get((start_idx + i) % file.len()).unwrap().1)
        .sum()
}

fn part1(mut file: Vec<(usize, i64)>) -> i64 {
    do_mixing(&mut file);
    get_coordinate_sum(&file)
}

fn part2(mut file: Vec<(usize, i64)>) -> i64 {
    file.iter_mut()
        .for_each(|(_, value)| *value *= DECRYPTION_KEY);
    for _ in 0..10 {
        do_mixing(&mut file);
    }
    get_coordinate_sum(&file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            1
            2
            -3
            3
            -2
            0
            4
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "3");
        assert_eq!(&part2, "1623178306");
    }
}
