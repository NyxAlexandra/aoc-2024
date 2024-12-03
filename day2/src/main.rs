#![feature(iter_map_windows)]

use std::convert::identity;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    parse_reports(input)
        .into_iter()
        .filter(|report| report_is_valid(report))
        .count()
}

fn part2(input: &str) -> usize {
    parse_reports(input)
        .into_iter()
        .filter(|report| {
            // report can also be valid if calling `report_is_valid` on any
            // subset of the report [0, i) ∪ (i, n) for any i in range [0, n)
            report_is_valid(report)
                || (0..report.len()).any(|i| {
                    report_is_valid(
                        report[..i].iter().chain(&report[(i + 1)..]),
                    )
                })
        })
        .count()
}

fn parse_reports(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

/// Returns `true` if all pairs of deltas in the report [are
/// valid](delta_pair_valid).
fn report_is_valid<'a>(report: impl IntoIterator<Item = &'a isize>) -> bool {
    report
        .into_iter()
        .map_windows(|[&level1, &level2]| level2 - level1)
        .map_windows(|[delta1, delta2]| delta_pair_valid(*delta1, *delta2))
        .all(identity)
}

/// Returns `true` if both deltas have the same sign and [are in the correct
/// range](delta_in_range).
fn delta_pair_valid(a: isize, b: isize) -> bool {
    a.signum() == b.signum() && delta_in_range(a) && delta_in_range(b)
}

/// Returns `true` if the delta is within the acceptable range [1, 3] by its
/// absolute value.
fn delta_in_range(delta: isize) -> bool {
    1 <= delta.abs() && delta.abs() <= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 4);
    }
}
