use std::collections::HashMap;

const ROOT_MONKEY: &str = "root";
const HUMN_MONKEY: &str = "humn";

enum Location {
    Left,
    Right,
    None,
}

#[derive(Hash)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unimplemented!(),
        }
    }

    fn execute(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }

    fn rev_ex(&self, lhs: i64, rhs: i64, switched: bool) -> i64 {
        match self {
            Self::Add => lhs - rhs,
            Self::Sub => {
                if switched {
                    return rhs - lhs;
                }
                lhs + rhs
            }
            Self::Mul => lhs / rhs,
            Self::Div => {
                if switched {
                    return rhs / lhs;
                }
                lhs * rhs
            }
        }
    }
}

#[derive(Hash)]
enum Monkey<'a> {
    Value(i64),
    Operation(&'a str, &'a str, Operator),
}

impl<'a> Monkey<'a> {
    fn new(s: &'a str) -> Self {
        if let Ok(value) = s.parse() {
            return Monkey::Value(value);
        }
        let mut op_parts = s.split(' ');
        let lhs = op_parts.next().unwrap();
        let operator = Operator::new(op_parts.next().unwrap());
        let rhs = op_parts.next().unwrap();
        Self::Operation(lhs, rhs, operator)
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &str) -> HashMap<&str, Monkey> {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let monkey = Monkey::new(parts.next().unwrap());
        monkeys.insert(key, monkey);
    }
    monkeys
}

fn find_result(current: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match monkeys.get(current).unwrap() {
        Monkey::Value(value) => *value,
        Monkey::Operation(lhs, rhs, operator) => {
            let lhs = find_result(lhs, monkeys);
            let rhs = find_result(rhs, monkeys);
            operator.execute(lhs, rhs)
        }
    }
}

fn calc_humn(current: &str, monkeys: &HashMap<&str, Monkey>, value: i64) -> i64 {
    if current == HUMN_MONKEY {
        return value;
    }
    if let Some(Monkey::Operation(lhs, rhs, operator)) = monkeys.get(current) {
        let (next, operand, switched) = match find_humn(current, monkeys) {
            Location::Left => (lhs, find_result(rhs, monkeys), false),
            Location::Right => (rhs, find_result(lhs, monkeys), true),
            Location::None => return 0,
        };
        return calc_humn(next, monkeys, operator.rev_ex(value, operand, switched));
    }
    0
}

fn find_humn(current: &str, monkeys: &HashMap<&str, Monkey>) -> Location {
    if let Some(Monkey::Operation(lhs, rhs, _)) = monkeys.get(current) {
        if contains_humn(lhs, monkeys) {
            return Location::Left;
        }
        if contains_humn(rhs, monkeys) {
            return Location::Right;
        }
    }
    Location::None
}

fn contains_humn(current: &str, monkeys: &HashMap<&str, Monkey>) -> bool {
    if current == HUMN_MONKEY {
        return true;
    }
    if let Some(Monkey::Operation(lhs, rhs, _)) = monkeys.get(current) {
        return contains_humn(lhs, monkeys) || contains_humn(rhs, monkeys);
    }
    false
}

fn part1(monkeys: HashMap<&str, Monkey>) -> i64 {
    find_result(ROOT_MONKEY, &monkeys)
}

fn part2(monkeys: HashMap<&str, Monkey>) -> i64 {
    if let Some(Monkey::Operation(lhs, rhs, _)) = monkeys.get(ROOT_MONKEY) {
        let (initial_monkey, value) = match find_humn(ROOT_MONKEY, &monkeys) {
            Location::Left => (lhs, find_result(rhs, &monkeys)),
            Location::Right => (rhs, find_result(lhs, &monkeys)),
            Location::None => return 0,
        };
        return calc_humn(initial_monkey, &monkeys, value);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            root: pppw + sjmn
            dbpl: 5
            cczh: sllz + lgvd
            zczc: 2
            ptdq: humn - dvpt
            dvpt: 3
            lfqf: 4
            humn: 5
            ljgn: 2
            sjmn: drzm * dbpl
            sllz: 4
            pppw: cczh / lfqf
            lgvd: ljgn * ptdq
            drzm: hmdt - zczc
            hmdt: 32
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "152");
        assert_eq!(&part2, "301");
    }
}
