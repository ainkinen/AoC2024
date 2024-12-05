use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MULTIPLY: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    static ref CONDITIONA_MULTIPLY: Regex =
        Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let mut total = 0;
    for cap in MULTIPLY.captures_iter(input) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        total += a * b;
    }
    total
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let caps = CONDITIONA_MULTIPLY.captures_iter(input);
    let mut multiply = true;
    let mut total = 0;
    for cap in caps {
        match &cap[0] {
            "do()" => multiply = true,
            "don't()" => multiply = false,
            _ => {
                if multiply {
                    let a = cap[1].parse::<i32>().unwrap();
                    let b = cap[2].parse::<i32>().unwrap();
                    total += a * b;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static CONDITIONAL_TEST_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(CONDITIONAL_TEST_INPUT), 48);
    }
}
