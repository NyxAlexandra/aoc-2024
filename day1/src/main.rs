use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (a, b) = parse_sorted_lists(input);

    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: &str) -> usize {
    let (a, b) = parse_sorted_lists(input);
    let counts = b.into_iter().counts();

    a.into_iter()
        .map(|n| {
            let count = counts.get(&n).copied().unwrap_or_default();

            n * count
        })
        .sum()
}

fn parse_sorted_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (mut a, mut b): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (usize::from_str(a).unwrap(), usize::from_str(b).unwrap()))
        .collect();

    a.sort_unstable();
    b.sort_unstable();

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 31);
    }
}
