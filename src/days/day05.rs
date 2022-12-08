#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }

    fn execute(&self, stacks: &mut [Vec<char>]) {
        for _ in 0..self.count {
            let el = stacks.get_mut(self.from).unwrap().pop().unwrap();
            stacks.get_mut(self.to).unwrap().push(el);
        }
    }

    fn execute_reverse(&self, stacks: &mut [Vec<char>]) {
        let mut group: Vec<char> = vec![];
        for _ in 0..self.count {
            let el = stacks.get_mut(self.from).unwrap().pop().unwrap();
            group.push(el);
        }
        group.reverse();
        stacks.get_mut(self.to).unwrap().append(&mut group);
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1, part2)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let input = input.lines().collect::<Vec<&str>>().join("|");
    let mut input = input.split("||");
    let mut arrangement: Vec<&str> = input.next().unwrap().split('|').collect();
    let instruction_texts: Vec<&str> = input.next().unwrap().split('|').collect();

    let cols: Vec<usize> = arrangement
        .pop()
        .unwrap()
        .trim()
        .split("   ")
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect();
    arrangement.reverse();

    let mut stacks = Vec::<Vec<char>>::new();

    for col in cols {
        let mut stack = Vec::<char>::new();
        for row in arrangement.iter() {
            let row: Vec<char> = row.chars().collect();
            let el = *row.get(1 + col * 4).unwrap();
            if el == ' ' {
                break;
            }
            stack.push(el);
        }
        stacks.push(stack);
    }

    let mut instructions = Vec::<Instruction>::new();
    for instruction in instruction_texts {
        let instruction = instruction
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");
        let mut instruction = instruction.split(' ').map(|x| x.parse::<usize>().unwrap());
        instructions.push(Instruction::new(
            instruction.next().unwrap(),
            instruction.next().unwrap() - 1,
            instruction.next().unwrap() - 1,
        ));
    }
    (stacks, instructions)
}

fn part1((mut stacks, instructions): (Vec<Vec<char>>, Vec<Instruction>)) -> String {
    for instruction in instructions {
        instruction.execute(&mut stacks);
    }
    stacks.iter().map(|x| x.last().unwrap()).collect::<String>()
}

fn part2((mut stacks, instructions): (Vec<Vec<char>>, Vec<Instruction>)) -> String {
    for instruction in instructions {
        instruction.execute_reverse(&mut stacks);
    }
    stacks.iter().map(|x| x.last().unwrap()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
                [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 
            
            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "CMZ");
        assert_eq!(&part2, "MCD");
    }
}
