use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Rule = (i32, i32);
type Rules = HashSet<Rule>;
type Update = Vec<i32>;

#[derive(Debug)]
struct Input {
    rules: Rules,
    updates: Vec<Update>,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    let sections: Vec<&str> = input.splitn(2, "\n\n").collect();
    let rules: Rules = sections[0]
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split('|').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let updates: Vec<Update> = sections[1]
        .lines()
        .map(|l| l.split(',').map(|t| t.parse::<i32>().unwrap()).collect())
        .collect();

    Input { rules, updates }
}

fn update_is_correct(update: &Update, rules: &Rules) -> bool {
    let pages: HashMap<i32, usize> = update.iter().enumerate().map(|(i, v)| (*v, i)).collect();

    rules.iter().all(|(l, r)| {
        if let (Some(l), Some(r)) = (pages.get(l), pages.get(r)) {
            l < r
        } else {
            true
        }
    })
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> i32 {
    let correct = input
        .updates
        .iter()
        .filter(|up| update_is_correct(up, &input.rules));

    correct
        .map(|up| {
            let middle = up.len() / 2;
            up[middle]
        })
        .sum()
}

fn fix(update: &Update, rules: &HashSet<Rule>) -> Update {
    let mut fixed = update.clone();
    'outer: loop {
        for i in 0..fixed.len() - 1 {
            let (l, r) = (fixed[i], fixed[i + 1]);
            if rules.contains(&(r, l)) {
                fixed.swap(i, i + 1);
                continue 'outer;
            }
        }
        break;
    }

    fixed
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> i32 {
    let incorrect = input
        .updates
        .iter()
        .filter(|up| !update_is_correct(up, &input.rules));

    let fixed = incorrect.map(|up| fix(up, &input.rules));

    fixed
        .map(|up| {
            let middle = up.len() / 2;
            up[middle]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "47|53
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
97,13,75,29,47
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 123);
    }
}
