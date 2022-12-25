const BASE: i64 = 5;

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));

    (part1, "Merry Christmas".to_owned())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

fn char_to_digit(c: &char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unimplemented!(),
    }
}

fn digit_to_char(d: i64) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unimplemented!(),
    }
}

fn from_snafu(snafu: &[char]) -> i64 {
    snafu.iter().fold(0, |s, c| s * 5 + char_to_digit(c))
}

fn to_snafu(mut decimal: i64) -> Vec<char> {
    let mut res = vec![];
    while decimal > 0 {
        let mut c = decimal % BASE;
        decimal /= BASE;
        if c >= 3 {
            decimal += 1;
            c -= BASE;
        }
        res.push(digit_to_char(c));
    }
    res
}

fn part1(fuel_requirements: Vec<Vec<char>>) -> String {
    to_snafu(
        fuel_requirements
            .iter()
            .map(|snafu| from_snafu(snafu))
            .sum(),
    )
    .iter()
    .rev()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            1=-0-2
            12111
            2=0=
            21
            2=01
            111
            20012
            112
            1=-1=
            1-12
            12
            1=
            122
        "};
        let (part1, _) = solve(input);

        assert_eq!(&part1, "2=-1=0");
    }
}
