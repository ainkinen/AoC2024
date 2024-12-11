use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;

type Stone = usize;

type Stones = Vec<Stone>;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Stones {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn split_stone(stone: &Stone) -> Option<Vec<Stone>> {
    let stone_as_string = stone.to_string();
    let digits = stone_as_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let digits_len = digits.len();
    if digits_len % 2 != 0 {
        // Not even, no split
        return None;
    }

    let (left_str, right_str) = stone_as_string.split_at(digits_len / 2);
    let left = left_str.parse().unwrap();
    let right = right_str.parse().unwrap();

    Some(vec![left, right])
}

#[cached]
fn count_recursive(stone: Stone, times: usize) -> usize {
    if times == 0 {
        return 1;
    }

    if stone == 0 {
        return count_recursive(1, times - 1);
    }

    if let Some(split_stones) = split_stone(&stone) {
        return split_stones
            .iter()
            .map(|&s| count_recursive(s, times - 1))
            .sum();
    }

    count_recursive(stone * 2024, times - 1)
}

#[aoc(day11, part1)]
fn part1(stones: &Stones) -> usize {
    stones.iter().map(|s| count_recursive(*s, 25)).sum()
}

#[aoc(day11, part2)]
fn part2(stones: &Stones) -> usize {
    stones.iter().map(|s| count_recursive(*s, 75)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 65601038650482);
    }
}
