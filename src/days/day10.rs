enum Instruction {
    Addx(isize),
    Noop,
}

struct Cpu {
    x: isize,
    cycle: isize,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, cycle: 1 }
    }

    fn execute_cycle(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(val) => self.x += *val,
        }
        self.cycle += 1;
    }

    fn render(&self) -> char {
        if (self.cycle - 1) % 40 >= self.x - 1 && (self.cycle - 1) % 40 <= self.x + 1 {
            return '#';
        }
        '.'
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|x| {
            let mut parts = x.split(' ');
            match parts.next().unwrap() {
                "addx" => vec![
                    Instruction::Noop,
                    Instruction::Addx(parts.next().unwrap().parse().unwrap()),
                ],
                "noop" => vec![Instruction::Noop],
                _ => unimplemented!(),
            }
        })
        .collect()
}

fn part1(instructions: Vec<Instruction>) -> isize {
    let mut cpu = Cpu::new();
    let mut sum = 0;
    for instruction in instructions {
        cpu.execute_cycle(&instruction);
        if (cpu.cycle + 20) % 40 == 0 {
            sum += cpu.cycle * cpu.x;
        }
    }
    sum
}

fn part2(instructions: Vec<Instruction>) -> String {
    let mut cpu = Cpu::new();
    let mut output: String = String::new();
    for instruction in instructions {
        output.push(cpu.render());
        cpu.execute_cycle(&instruction);
        if (cpu.cycle - 1) % 40 == 0 {
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "13140");
        assert_eq!(
            &part2,
            indoc! {"
                ##..##..##..##..##..##..##..##..##..##..
                ###...###...###...###...###...###...###.
                ####....####....####....####....####....
                #####.....#####.....#####.....#####.....
                ######......######......######......####
                #######.......#######.......#######.....
            "}
        );
    }
}
