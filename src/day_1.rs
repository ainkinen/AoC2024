use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::zip;

struct Lists {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse_line(line: &str) -> (i32, i32) {
    if let [Ok(a), Ok(b)] = line
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Vec<_>>()[..]
    {
        return (a, b);
    }

    panic!("Failed to parse line: {}", line);
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Lists {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let (a, b) = parse_line(line);
        left.push(a);
        right.push(b);
    }

    Lists { left, right }
}

#[aoc(day1, part1)]
fn part1(input_lists: &Lists) -> i32 {
    let mut left = input_lists.left.clone();
    left.sort();
    let mut right = input_lists.right.clone();
    right.sort();

    let mut total = 0;

    for (l, r) in zip(left, right) {
        total += (l - r).abs();
    }

    total
}

fn count_entries(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for item in list {
        *counts.entry(*item).or_insert(0) += 1;
    }

    counts
}

#[aoc(day1, part2)]
fn part2(input_lists: &Lists) -> i32 {
    let counts = count_entries(&input_lists.right);

    input_lists
        .left
        .iter()
        .fold(0, |acc, x| acc + x * counts.get(x).unwrap_or(&0))
    // 0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(part2(&input), 31);
    }

    #[test]
    fn test_count_entries() {
        let list = vec![1, 2, 3, 4, 5, 3, 4, 3];
        let count = count_entries(&list);
        assert_eq!(
            count,
            HashMap::from([(1, 1), (2, 1), (3, 3), (4, 2), (5, 1)])
        )
    }
}
