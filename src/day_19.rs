use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use cached::UnboundCache;
use regex::bytes::Regex;
use std::hash::{DefaultHasher, Hash, Hasher};

struct Input {
    towels: Vec<String>,
    patterns: Vec<String>,
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.trim().to_string())
        .collect();

    lines.next(); // divider

    let patterns = lines.map(|l| l.trim().to_string()).collect();

    Input { towels, patterns }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let regex_any = input.towels.join("|");
    let regex: Regex = Regex::new(&format!("^({regex_any})+$")).unwrap();

    input
        .patterns
        .iter()
        .filter(|p| regex.is_match(p.as_ref()))
        .count()
}

fn hash(string: &str, col: &Vec<String>) -> u64 {
    let mut hasher = DefaultHasher::new();
    col.hash(&mut hasher);
    string.hash(&mut hasher);
    hasher.finish()
}

#[cached(
    ty = "UnboundCache<u64, usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ hash(string, patterns) }"#
)]
fn count_ways(string: &str, patterns: &Vec<String>) -> usize {
    let mut ways = 0;

    for pattern in patterns.clone() {
        if *string == pattern {
            ways += 1;
        }
        if let Some(remainder) = string.strip_prefix(&pattern) {
            ways += count_ways(remainder, patterns);
        }
    }

    ways
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    input
        .patterns
        .iter()
        .map(|p| count_ways(p, &input.towels))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 16);
    }
}
