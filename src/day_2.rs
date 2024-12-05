use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::iter::Iterator;

type Report = Vec<i32>;

fn parse_line(l: &str) -> Report {
    l.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn is_increasing(report: &Report) -> bool {
    report
        .iter()
        .tuple_windows()
        .all(|chunk: (&i32, &i32)| chunk.0 < chunk.1)
}

fn is_decreasing(report: &Report) -> bool {
    report
        .iter()
        .tuple_windows()
        .all(|chunk: (&i32, &i32)| chunk.0 > chunk.1)
}

fn diff_in_range(report: &Report) -> bool {
    report
        .iter()
        .tuple_windows()
        .all(|chunk: (&i32, &i32)| -> bool {
            let diff = (chunk.0 - chunk.1).abs();
            (1..=3).contains(&diff)
        })
}

fn is_safe(report: &Report) -> bool {
    (is_increasing(report) || is_decreasing(report)) && diff_in_range(report)
}

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    input.lines().map(parse_line).filter(is_safe).count() as i32
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(parse_line)
        .map(|report| -> Vec<Report> {
            (0..report.len())
                .map(|i| {
                    let mut clone = report.to_vec();
                    clone.remove(i);
                    clone
                })
                .collect()
        })
        .filter(|reports| reports.iter().any(is_safe))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_parse_report() {
        assert_eq!(parse_line("7 6 4 2 1"), vec![7, 6, 4, 2, 1]);
        assert_eq!(parse_line("1 2 7 8 9"), vec![1, 2, 7, 8, 9]);
        assert_eq!(parse_line("9 7 6 2 1"), vec![9, 7, 6, 2, 1]);
        assert_eq!(parse_line("1 3 2 4 5"), vec![1, 3, 2, 4, 5]);
        assert_eq!(parse_line("8 6 4 4 1"), vec![8, 6, 4, 4, 1]);
        assert_eq!(parse_line("1 3 6 7 9"), vec![1, 3, 6, 7, 9]);
    }

    #[test]
    fn test_is_increasing() {
        assert!(is_increasing(&vec![1, 2, 7, 8, 9]));
        assert!(!is_increasing(&vec![5, 4, 3, 2, 1]));
        assert!(!is_increasing(&vec![1, 2, 2, 3, 4]));
    }

    #[test]
    fn test_is_decreasing() {
        assert!(is_decreasing(&vec![5, 4, 3, 2, 1]));
        assert!(!is_decreasing(&vec![1, 2, 7, 8, 9]));
        assert!(!is_decreasing(&vec![5, 4, 3, 3, 2, 1]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
