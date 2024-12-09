use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (rules, lists) = parse_print_queue(input);

    lists
        .into_iter()
        .filter(|list| is_sorted(&rules, list))
        .map(|list| list[list.len() / 2])
        .sum()
}

fn part2(input: &str) -> usize {
    let (rules, lists) = parse_print_queue(input);

    lists
        .into_iter()
        .filter(|list| !is_sorted(&rules, list))
        .map(|mut list| {
            sort(&rules, &mut list);

            list[list.len() / 2]
        })
        .sum()
}

type Rules = HashMap<usize, HashSet<usize>>;

fn parse_print_queue(s: &str) -> (Rules, Vec<Vec<usize>>) {
    let (rules, lists) = s.split_once("\n\n").unwrap();

    (
        {
            let mut ret = Rules::new();

            // can't use `Iterator::collect` because each before can associate
            // with multiple afters
            for (before, after) in rules
                .lines()
                .flat_map(|line| line.split_once('|'))
                .flat_map(|(before, after)| {
                    before.parse().ok().zip(after.parse().ok())
                })
            {
                ret.entry(before).or_default().insert(after);
            }

            ret
        },
        lists
            .lines()
            .map(|line| line.split(',').flat_map(str::parse))
            .map(Iterator::collect)
            .collect(),
    )
}

fn is_sorted(rules: &Rules, list: &[usize]) -> bool {
    list.is_sorted_by(cmp(rules))
}

fn sort(rules: &Rules, list: &mut [usize]) {
    list.sort_by(|a, b| cmp(rules)(a, b).cmp(&true));
}

fn cmp(rules: &Rules) -> impl Fn(&usize, &usize) -> bool + use<'_> {
    |a, b| rules.get(a).map(|after| after.contains(b)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 123);
    }
}
