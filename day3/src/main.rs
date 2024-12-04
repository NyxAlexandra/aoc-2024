use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let parser = Parser::new();

    parser
        .parse(input)
        .flat_map(|op| {
            if let Op::Mul(lhs, rhs) = op {
                Some(lhs * rhs)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let parser = Parser::new();

    parser
        .parse(input)
        .scan(true, |on, op| match op {
            Op::Do => {
                *on = true;

                Some(None)
            },
            Op::Dont => {
                *on = false;

                Some(None)
            },
            Op::Mul(lhs, rhs) => Some(on.then_some(lhs * rhs)),
        })
        .flatten()
        .sum()
}

struct Parser {
    regex: Regex,
}

enum Op {
    Do,
    Dont,
    Mul(usize, usize),
}

impl Parser {
    fn new() -> Self {
        // cannot panic as the regex is valid
        let regex =
            Regex::new("(mul\\(\\d+,\\d+\\)|do\\(\\)|don't\\(\\))").unwrap();

        Self { regex }
    }

    fn parse<'a>(
        &self,
        input: &'a str,
    ) -> impl Iterator<Item = Op> + use<'_, 'a> {
        self.regex.find_iter(input).map(|mat| {
            // none of these can panic due to the conditions of the regex

            let (op, args) = mat.as_str().split_once('(').unwrap();
            let args = args.strip_suffix(')').unwrap();

            match op {
                "do" => Op::Do,
                "don't" => Op::Dont,
                "mul" => {
                    let (lhs, rhs) = args.split_once(',').unwrap();
                    let (lhs, rhs) =
                        (lhs.parse().unwrap(), rhs.parse().unwrap());

                    Op::Mul(lhs, rhs)
                },
                _ => unreachable!(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,\
                     64]then(mul(11,8)mul(8,5))";

        assert_eq!(part1(input), 161);
    }

    #[test]
    fn example2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,\
                     8)undo()?mul(8,5))";

        assert_eq!(part2(input), 48);
    }
}
