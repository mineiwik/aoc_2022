use std::{cmp::Ordering, slice::Iter, str::Chars};

#[derive(Debug, Clone, PartialEq)]
enum Content {
    Value(usize),
    List(Vec<Content>),
}

fn cmp_list(mut lhs: Iter<Content>, mut rhs: Iter<Content>) -> Ordering {
    loop {
        let result = match (lhs.next(), rhs.next()) {
            (None, None) => return Ordering::Equal,
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (Some(lhs), Some(rhs)) => lhs.cmp(rhs),
        };
        if result != Ordering::Equal {
            return result;
        }
    }
}

impl Content {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Self::Value(lhs_val), Self::Value(rhs_val)) => lhs_val.cmp(rhs_val),
            (Self::Value(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(rhs),
            (Self::List(_), Self::Value(_)) => self.cmp(&Self::List(vec![rhs.clone()])),
            (Self::List(lhs), Self::List(rhs)) => cmp_list(lhs.iter(), rhs.iter()),
        }
    }
}

pub fn solve(input: &str) -> (String, String) {
    let part1 = part1(parse_input(input));
    let part2 = part2(parse_input(input));

    (part1.to_string(), part2.to_string())
}

fn parse_packet(packet: &mut Chars) -> Vec<Content> {
    let mut values = vec![];
    let mut content = String::new();
    while let Some(next_val) = packet.next() {
        match next_val {
            '0'..='9' => content.push(next_val),
            _ => {
                if let Ok(res) = content.parse() {
                    values.push(Content::Value(res));
                }
                content = String::new();
            }
        }
        match next_val {
            '[' => values.push(Content::List(parse_packet(packet))),
            ']' => break,
            _ => {}
        }
    }
    values
}

fn parse_input(input: &str) -> Vec<Content> {
    let input = input.lines().collect::<Vec<&str>>().join("|");
    input
        .split("||")
        .flat_map(|x| {
            x.split('|')
                .flat_map(|y| parse_packet(&mut y.chars()))
                .collect::<Vec<Content>>()
        })
        .collect()
}

fn part1(pairs: Vec<Content>) -> usize {
    let mut sum = 0;
    for (idx, pair) in pairs.chunks(2).enumerate() {
        if pair[0].cmp(&pair[1]) == Ordering::Less {
            sum += idx + 1;
        }
    }
    sum
}

fn part2(mut pairs: Vec<Content>) -> usize {
    let first = Content::List(vec![Content::List(vec![Content::Value(2)])]);
    let second = Content::List(vec![Content::List(vec![Content::Value(6)])]);
    pairs.push(first.clone());
    pairs.push(second.clone());
    pairs.sort_by(|lhs, rhs| lhs.cmp(rhs));
    let mut prod = 1;
    for (idx, packet) in pairs.iter().enumerate() {
        if *packet == first || *packet == second {
            prod *= idx + 1;
        }
    }
    prod
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            [1,1,3,1,1]
            [1,1,5,1,1]
            
            [[1],[2,3,4]]
            [[1],4]
            
            [9]
            [[8,7,6]]
            
            [[4,4],4,4]
            [[4,4],4,4,4]
            
            [7,7,7,7]
            [7,7,7]
            
            []
            [3]
            
            [[[]]]
            [[]]
            
            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "};
        let (part1, part2) = solve(input);

        assert_eq!(&part1, "13");
        assert_eq!(&part2, "140");
    }
}
