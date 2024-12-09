use aoc_runner_derive::{aoc, aoc_generator};

struct Input {}

#[aoc_generator(dayX)]
fn parse_input(input: &str) -> Input {
    let _ = input;
    Input {}
}

#[aoc(dayx, part1)]
fn part1(input: &Input) -> i32 {
    let _ = input;
    0
}

#[aoc(dayx, part2)]
fn part2(input: &Input) -> i32 {
    let _ = input;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 0);
    }
}
