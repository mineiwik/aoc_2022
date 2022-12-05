const POINTS_WON: usize = 6;
const POINTS_DRAW: usize = 3;
const POINTS_LOST: usize = 0;

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let rounds: Vec<&str> = input.lines().collect();
    let mut strategies: Vec<(usize, usize)> = Vec::new();

    for round in rounds {
        let round = round.replace(" ", "");
        let mut signs = round.chars();
        let mut opponent = signs.next().unwrap() as usize;
        let mut response = signs.next().unwrap() as usize;
        opponent -= 'A' as usize;
        response -= 'X' as usize;
        strategies.push((opponent, response));
    }
    strategies
}

fn part1(strategies: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for (opponent, player) in strategies {
        sum += get_points(player, opponent);
    }
    sum
}

fn part2(strategies: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for (opponent, outcome) in strategies {
        let player = get_player_sign(opponent, outcome);
        sum += get_points(player, opponent);
    }
    sum
}

fn get_points(player: usize, opponent: usize) -> usize {
    let mut sum = get_outcome_points(player, opponent);
    sum += player + 1;
    sum
}

fn get_player_sign(opponent: usize, outcome: usize) -> usize {
    (opponent + outcome + 2) % 3
}

fn get_outcome_points(player: usize, opponent: usize) -> usize {
    if player == opponent {
        return POINTS_DRAW;
    }
    if player == (opponent + 1) % 3 {
        return POINTS_WON;
    }
    return POINTS_LOST;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
        A Y
        B X
        C Z
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "15");
        assert_eq!(&part2, "12");
    }
}
