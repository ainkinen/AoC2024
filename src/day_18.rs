use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

type YX = (i32, i32);

type Drops = Vec<YX>;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Drops {
    let numbers = NUMBER
        .captures_iter(input)
        .map(|c| c[0].parse::<i32>().unwrap());

    let mut drops = vec![];
    for (x, y) in numbers.tuples() {
        drops.push((y, x));
    }

    drops
}

fn bfs(start: &YX, goal: &YX, drops: &[YX], size: i32) -> Option<usize> {
    let area = 0..size;

    let corruptions: HashSet<&YX> = HashSet::from_iter(drops.iter());

    let mut visited = HashSet::new();
    let mut trails = VecDeque::from([vec![*start]]);

    while let Some(trail) = trails.pop_front() {
        let head = trail.last().unwrap();
        if head == goal {
            return Some(trail.len());
        }

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for dir in dirs {
            let next = (head.0 + dir.0, head.1 + dir.1);
            if !corruptions.contains(&next)
                && area.contains(&next.0)
                && area.contains(&next.1)
                && !visited.contains(&next)
            {
                let mut new_trail = trail.clone();
                new_trail.push(next);
                trails.push_back(new_trail);
                visited.insert(next);
            }
        }
    }

    None
}

fn part1_solver(drops: &Drops, num_drops: usize, size: i32) -> usize {
    let trail_length = bfs(&(0, 0), &(size - 1, size - 1), &drops[0..num_drops], size).unwrap();

    // Ignore starting point
    trail_length - 1
}

#[allow(dead_code)]
fn part2_solver(drops: &Drops, size: i32) -> String {
    for i in 1..drops.len() {
        let trail_length = bfs(&(0, 0), &(size - 1, size - 1), &drops[0..i], size);

        if trail_length.is_none() {
            let byte = drops[i - 1];
            return format!("{},{}", byte.1, byte.0);
        }
    }

    panic!("No solution found");
}

fn part2_solver_binary_search(drops: &Drops, size: i32) -> String {
    let items = Vec::from_iter(0..drops.len());

    let partition_point = items.partition_point(|num_drops| {
        bfs(&(0, 0), &(size - 1, size - 1), &drops[0..*num_drops], size).is_some()
    });

    let byte = drops[partition_point - 1];
    format!("{},{}", byte.1, byte.0)
}

#[aoc(day18, part1)]
fn part1(drops: &Drops) -> usize {
    part1_solver(drops, 1024, 71)
}

// #[aoc(day18, part2, slow)]
// fn part2(drops: &Drops) -> String {
//     part2_solver(drops, 71)
// }

#[aoc(day18, part2, binary_search)]
fn part2_binary_search(drops: &Drops) -> String {
    part2_solver_binary_search(drops, 71)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part1() {
        assert_eq!(part1_solver(&parse_input(TEST_INPUT), 12, 7), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_solver(&parse_input(TEST_INPUT), 7), "6,1");
    }

    #[test]
    fn test_part2_binary_search() {
        assert_eq!(
            part2_solver_binary_search(&parse_input(TEST_INPUT), 7),
            "6,1"
        );
    }
}
