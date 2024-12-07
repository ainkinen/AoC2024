use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Equation {
    target: i64,
    values: Vec<i64>,
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (target, rest) = line.split(':').collect_tuple().expect("bad input");

            let values = rest
                .split_whitespace()
                .map(|n| n.parse().expect("bad input"))
                .collect();

            Equation {
                target: target.trim().parse().unwrap(),
                values,
            }
        })
        .collect()
}

fn is_possible(eq: &Equation) -> bool {
    let (first, rest) = eq.values.split_first().unwrap();

    let acc = HashSet::from([*first]);

    let possible_results: HashSet<i64> = rest.iter().fold(acc, |acc, cur| {
        acc.iter().flat_map(|&v| [v + cur, v * cur]).collect()
    });
    possible_results.contains(&eq.target)
}

fn is_possible_with_concat(eq: &Equation) -> bool {
    let (first, rest) = eq.values.split_first().unwrap();

    let acc = HashSet::from([*first]);

    let possible_results: HashSet<i64> = rest.iter().fold(acc, |acc, cur| {
        acc.iter()
            .flat_map(|&v| {
                [v + cur, v * cur, {
                    let s = format!("{}{}", v, cur);
                    s.parse().unwrap()
                }]
            })
            .collect()
    });
    possible_results.contains(&eq.target)
}

fn is_possible_with_concat_faster(eq: &Equation) -> bool {
    let (first, rest) = eq.values.split_first().unwrap();

    let acc = HashSet::from([*first]);

    let possible_results: HashSet<i64> = rest.iter().fold(acc, |acc, cur| {
        acc.iter()
            .flat_map(|&v| {
                [v + cur, v * cur, {
                    let num_digits = cur.ilog10() + 1;
                    let multiplier = 10_i64.pow(num_digits);
                    v * multiplier + cur
                }]
            })
            .collect()
    });
    possible_results.contains(&eq.target)
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> i64 {
    input
        .iter()
        .filter(|eq| is_possible(eq))
        .map(|eq| eq.target)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Equation>) -> i64 {
    input
        .iter()
        .filter(|eq| is_possible_with_concat(eq))
        .map(|eq| eq.target)
        .sum()
}

#[aoc(day7, part2, faster)]
fn part2_faster(input: &Vec<Equation>) -> i64 {
    input
        .iter()
        .filter(|eq| is_possible_with_concat_faster(eq))
        .map(|eq| eq.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 11387);
    }

    #[test]
    fn test_part2_faster() {
        assert_eq!(part2_faster(&parse_input(TEST_INPUT)), 11387);
    }
}
