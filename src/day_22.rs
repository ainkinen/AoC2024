use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Input = Vec<u64>;

struct SecretNumberGenerator {
    current: u64,
}

impl Iterator for SecretNumberGenerator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let next_value = next_number(self.current);
        self.current = next_value;
        Some(next_value)
    }
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn next_number(number: u64) -> u64 {
    let mut n = number as u128;

    n = (n << 6) ^ n; // *64 and mix
    n %= 16777216; // prune

    n = (n >> 5) ^ n; // /32 and mix
    n %= 16777216; // prune

    n = (n << 11) ^ n; // *2048 and mix
    n %= 16777216; // prune

    n as u64
}

#[aoc(day22, part1)]
fn part1(numbers: &Input) -> u64 {
    let nth_numbers = numbers.iter().map(|n| {
        let gen = SecretNumberGenerator { current: *n };
        gen.into_iter().nth(1999).unwrap()
    });
    nth_numbers.sum()
}

type Seq = (i8, i8, i8, i8);
type MonkeyMap = HashMap<Seq, i8>; // seq to value at first seen

fn map_monkey(number: u64) -> MonkeyMap {
    let mut map = HashMap::new();

    let gen = SecretNumberGenerator { current: number };

    gen.take(2000)
        .map(|n| (n % 10) as i8)
        .tuple_windows()
        .map(|(val1, val2)| (val2, val2 - val1))
        .tuple_windows()
        .for_each(|((_v1, c1), (_v2, c2), (_v3, c3), (v4, c4))| {
            let changes = (c1, c2, c3, c4);
            map.entry(changes).or_insert(v4);
        });

    map
}

#[aoc(day22, part2)]
fn part2(input: &Input) -> u64 {
    let monkey_maps = input.iter().map(|number| map_monkey(*number)).collect_vec();

    let all_seen_sequences = monkey_maps
        .iter()
        .flat_map(|m| m.keys())
        .collect::<HashSet<_>>();

    all_seen_sequences
        .iter()
        .map(|seq| {
            monkey_maps
                .iter()
                .flat_map(|m| m.get(seq))
                .map(|v| *v as u64)
                .sum()
        })
        .max()
        .unwrap()
}

type Counter = HashMap<Seq, usize>;

#[aoc(day22, part2, faster)]
fn part2_faster(input: &Input) -> usize {
    let mut counter: Counter = HashMap::new();

    for number in input.iter() {
        let gen = SecretNumberGenerator { current: *number };

        let mut seen = HashSet::new();

        gen.take(2000)
            .map(|n| (n % 10) as i8)
            .tuple_windows()
            .map(|(val1, val2)| (val2, val2 - val1))
            .tuple_windows()
            .for_each(|((_, c1), (_, c2), (_, c3), (v4, c4))| {
                let seq = (c1, c2, c3, c4);
                let not_seen_yet = seen.insert(seq);
                if not_seen_yet {
                    *counter.entry(seq).or_insert(0) += v4 as usize;
                }
            });
    }

    *counter.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "1
10
100
2024
";

    static TEST_INPUT_2: &str = "1
2
3
2024
";

    #[test]
    fn test_secret_number_generator() {
        let gen = SecretNumberGenerator { current: 123 };
        let mut it = gen.into_iter();

        assert_eq!(it.next().unwrap(), 15887950);
        assert_eq!(it.next().unwrap(), 16495136);
        assert_eq!(it.next().unwrap(), 527345);
        assert_eq!(it.next().unwrap(), 704524);
        assert_eq!(it.next().unwrap(), 1553684);
        assert_eq!(it.next().unwrap(), 12683156);
        assert_eq!(it.next().unwrap(), 11100544);
        assert_eq!(it.next().unwrap(), 12249484);
        assert_eq!(it.next().unwrap(), 7753432);
        assert_eq!(it.next().unwrap(), 5908254);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 23);
    }

    #[test]
    fn test_part2_faster() {
        assert_eq!(part2_faster(&parse_input(TEST_INPUT_2)), 23);
    }
}
