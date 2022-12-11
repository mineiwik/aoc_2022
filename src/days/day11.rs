use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Number(usize),
    ItemValue,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operator: Operator,
    operand: Operand,
    test: usize,
    result: (usize, usize),
}

impl Operator {
    fn compute(&self, lhs: usize, rhs: usize, divide: bool) -> usize {
        let intermediate_res = match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
        };
        if divide {
            return intermediate_res / 3;
        }
        intermediate_res
    }
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operator: Operator,
        operand: Operand,
        test: usize,
        result: (usize, usize),
    ) -> Self {
        Self {
            items,
            operator,
            operand,
            test,
            result,
        }
    }

    fn throw_everything(&mut self, divide: bool) -> Vec<(usize, usize)> {
        let mut throw_results = vec![];
        for item in self.items.clone() {
            throw_results.push(self.get_throw_result(item, divide));
        }
        self.items = vec![];
        throw_results
    }

    fn get_throw_result(&self, item: usize, divide: bool) -> (usize, usize) {
        let operand = match self.operand {
            Operand::Number(val) => val,
            Operand::ItemValue => item,
        };
        let item = self.operator.compute(item, operand, divide);
        (self.get_next_monkey(item), item)
    }

    fn get_next_monkey(&self, item: usize) -> usize {
        if item % self.test == 0 {
            return self.result.0;
        }
        self.result.1
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_after<T: FromStr>(input: &str, replace: &str) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    input.replace(replace, "").trim().parse().unwrap()
}

fn parse_input(input: &str) -> (Vec<Monkey>, usize) {
    let input = input.lines().collect::<Vec<&str>>().join("|");
    let notes: Vec<Vec<&str>> = input.split("||").map(|x| x.split('|').collect()).collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut common_multiple = 1;
    for note in notes {
        let mut note = note.iter().skip(1);
        let items: String = parse_after(note.next().unwrap(), "Starting items: ");
        let items: Vec<usize> = items.split(", ").map(|x| x.parse().unwrap()).collect();
        let operation: String = parse_after(note.next().unwrap(), "Operation: new = old ");
        let mut operation = operation.split(' ');
        let operator = match operation.next().unwrap() {
            "*" => Operator::Mul,
            "+" => Operator::Add,
            _ => unimplemented!(),
        };
        let operand = match operation.next().unwrap().parse::<usize>() {
            Result::Ok(val) => Operand::Number(val),
            Result::Err(_) => Operand::ItemValue,
        };
        let test: usize = parse_after(note.next().unwrap(), "Test: divisible by ");
        let positive: usize = parse_after(note.next().unwrap(), "If true: throw to monkey ");
        let negative: usize = parse_after(note.next().unwrap(), "If false: throw to monkey ");
        let monkey = Monkey::new(items, operator, operand, test, (positive, negative));
        common_multiple *= test;
        monkeys.push(monkey);
    }
    (monkeys, common_multiple)
}

fn simulate_monkeys(
    (mut monkeys, common_multiple): (Vec<Monkey>, usize),
    rounds: usize,
    divide: bool,
) -> usize {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            *inspections.get_mut(i).unwrap() += monkey.items.len();
            for (next_monkey, mut item) in monkey.throw_everything(divide) {
                let next_monkey = monkeys.get_mut(next_monkey).unwrap();
                if !divide {
                    item %= common_multiple;
                }
                next_monkey.items.push(item);
            }
        }
    }
    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn part1(input: (Vec<Monkey>, usize)) -> usize {
    simulate_monkeys(input, 20, true)
}

fn part2(input: (Vec<Monkey>, usize)) -> usize {
    simulate_monkeys(input, 10_000, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            
            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0
            
            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3
            
            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "10605");
        assert_eq!(&part2, "2713310158");
    }
}
