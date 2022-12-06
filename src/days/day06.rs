use bevy::utils::HashSet;

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1, part2)
}

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn is_different_window(window: &[char]) -> bool {
    let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
    set.len() == window.len()
}

fn get_len_to_start_marker(stream: &Vec<char>, w_size: usize) -> Option<usize> {
    let mut window_end = w_size;
    while window_end <= stream.len() {
        if is_different_window(&stream[window_end - w_size..window_end]) {
            return Some(window_end);
        }
        window_end += 1;
    }
    None
}

fn part1(stream: Vec<char>) -> String {
    if let Some(res) = get_len_to_start_marker(&stream, 4) {
        return res.to_string();
    }
    "NO SOLUTION FOUND!".to_string()
}

fn part2(stream: Vec<char>) -> String {
    if let Some(res) = get_len_to_start_marker(&stream, 14) {
        return res.to_string();
    }
    "NO SOLUTION FOUND!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            mjqjpqmgbljsphdztnvjfqwrcgsmlb
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "7");
        assert_eq!(&part2, "19");
    }
}
