use std::collections::HashMap;

const MAX_CAPACITY: usize = 70000000;
const SPACE_REQUIRED: usize = 30000000;
const MAX_DIR_SIZE: usize = 100000;

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|x| x.split(' ').collect()).collect()
}

fn leave_directory(dirs: &mut Vec<String>, dir_sums: &mut HashMap<String, usize>) {
    let amount = *dir_sums.get(dirs.join(";").as_str()).unwrap();
    dirs.pop();
    *dir_sums.get_mut(dirs.join(";").as_str()).unwrap() += amount;
}

fn enter_directory(dirs: &mut Vec<String>, dir_sums: &mut HashMap<String, usize>, dir: String) {
    dirs.push(dir);
    if !dir_sums.contains_key(dirs.join(";").as_str()) {
        dir_sums.insert(dirs.join(";"), 0);
    }
}

fn get_sizes(output: Vec<Vec<&str>>) -> HashMap<String, usize> {
    let mut dirs = Vec::<String>::new();
    let mut dir_sums = HashMap::<String, usize>::new();
    for line in output {
        match *line.first().unwrap() {
            "$" => match *line.get(1).unwrap() {
                "cd" => match *line.get(2).unwrap() {
                    ".." => leave_directory(&mut dirs, &mut dir_sums),
                    dir => enter_directory(&mut dirs, &mut dir_sums, dir.to_string()),
                },
                "ls" => {}
                _ => unimplemented!(),
            },
            "dir" => {}
            file_size => {
                *dir_sums.get_mut(dirs.join(";").as_str()).unwrap() +=
                    file_size.parse::<usize>().unwrap();
            }
        }
    }
    while dirs.len() > 1 {
        leave_directory(&mut dirs, &mut dir_sums);
    }
    dir_sums
}

fn part1(output: Vec<Vec<&str>>) -> usize {
    let dir_sums = get_sizes(output);
    let sum = dir_sums
        .values()
        .filter(|x| **x <= MAX_DIR_SIZE)
        .fold(0, |acc, x| acc + *x);
    sum
}

fn part2(output: Vec<Vec<&str>>) -> usize {
    let dir_sums = get_sizes(output);
    let total: usize = *dir_sums.get("/").unwrap();
    let search_size = SPACE_REQUIRED - (MAX_CAPACITY - total);
    let mut dirs: Vec<usize> = dir_sums
        .values()
        .cloned()
        .filter(|x| *x >= search_size)
        .collect();
    dirs.sort();
    *dirs.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "95437");
        assert_eq!(&part2, "24933642");
    }
}
