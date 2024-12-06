#![feature(strict_overflow_ops, array_try_map)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);

    grid.keys()
        .flat_map(|[x, y]| {
            CROSS_DELTAS.into_iter().filter(|[dx, dy]| {
                (0..).zip("XMAS".chars()).all(|(i, char)| {
                    x.checked_add_signed(dx * i)
                        .zip(y.checked_add_signed(dy * i))
                        .map(|(x, y)| grid.get(&[x, y]) == Some(&char))
                        .unwrap_or_default()
                })
            })
        })
        .count()
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);

    grid.iter()
        .filter_map(|(cell, char)| (*char == 'A').then_some(cell))
        .flat_map(|[x, y]| {
            X_DELTAS.try_map(|[dx, dy]| {
                x.checked_add_signed(dx)
                    .zip(y.checked_add_signed(dy))
                    .and_then(|(x, y)| grid.get(&[x, y]))
            })
        })
        .filter(|[ne, se, sw, nw]| {
            [[ne, sw], [nw, se]].into_iter().all(|[start, end]| {
                matches!((start, end), ('M', 'S') | ('S', 'M'))
            })
        })
        .count()
}

/// Array of `[dx, dy]` offsets for a crossword.
const CROSS_DELTAS: [[isize; 2]; 8] = [
    // north
    [0, -1],
    // north-east
    [1, -1],
    // east
    [1, 0],
    // south-east
    [1, 1],
    // south
    [0, 1],
    // south-west
    [-1, 1],
    // west
    [-1, 0],
    // north-west
    [-1, -1],
];

/// Array of `[dx, dy]` offsets for X shapes.
const X_DELTAS: [[isize; 2]; 4] = [
    // north-east
    [1, -1],
    // south-east
    [1, 1],
    // south-west
    [-1, 1],
    // north-west
    [-1, -1],
];

fn parse_grid(input: &str) -> HashMap<[usize; 2], char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| (([x, y], char)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    // `rustfmt` breaks the newlines
    #[rustfmt::skip]
        const INPUT: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\n\
                    XMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\n\
                     MAMMMXMMMM\nMXMXAXMASX\n";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 9);
    }
}
