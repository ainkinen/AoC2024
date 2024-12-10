use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Map = Vec<Vec<i32>>;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let number = c.to_digit(10);
                    if let Some(number) = number {
                        number as i32
                    } else {
                        -1
                    }
                })
                .collect()
        })
        .collect()
}

fn get_neighbors(map: &Map, y: i32, x: i32) -> Vec<(i32, i32)> {
    let cur_val = map[y as usize][x as usize];

    let range = 0..map.len() as i32;

    [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
        .into_iter()
        .filter(|(y, x)| range.contains(y) && range.contains(x))
        .filter(|&(y, x)| map[y as usize][x as usize] == cur_val + 1)
        .collect()
}

fn count_reached_summits(map: &Map, y: i32, x: i32) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::from([(y, x)]);

    let mut heads = HashSet::<(i32, i32)>::from([(y, x)]);

    while !heads.is_empty() {
        heads = heads
            .iter()
            .flat_map(|&(y, x)| get_neighbors(map, y, x))
            .filter(|&(y, x)| !visited.contains(&(y, x)))
            .collect();

        visited.extend(&heads);
    }

    visited
        .iter()
        .filter(|&&(y, x)| map[y as usize][x as usize] == 9)
        .count() as u32
}

fn count_summit_trails(map: &Map, y: i32, x: i32) -> u32 {
    let mut completed_trails = HashSet::<Vec<(i32, i32)>>::new();

    let mut trails = HashSet::<Vec<(i32, i32)>>::from([vec![(y, x)]]);

    while !trails.is_empty() {
        trails = trails
            .iter()
            .flat_map(|trail| {
                let &(y, x) = trail.last().unwrap();
                let next_steps = get_neighbors(map, y, x);
                next_steps
                    .iter()
                    .map(|&h| {
                        let mut clone = trail.clone();
                        clone.push(h);
                        clone
                    })
                    .collect::<Vec<Vec<(i32, i32)>>>()
            })
            .collect();

        for trail in trails.iter() {
            let &(y, x) = trail.last().unwrap();
            if map[y as usize][x as usize] == 9 {
                completed_trails.insert(trail.clone());
            }
        }
    }

    completed_trails.len() as u32
}

#[aoc(day10, part1)]
fn part1(map: &Map) -> u32 {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| {
                    if map[y][x] == 0 {
                        count_reached_summits(map, y as i32, x as i32)
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[aoc(day10, part2)]
fn part2(map: &Map) -> u32 {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| {
                    if map[y][x] == 0 {
                        count_summit_trails(map, y as i32, x as i32)
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    static TEST_INPUT_2: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
";

    #[test]
    fn test_calculate_trailhead_score() {
        let map = parse_input(TEST_INPUT);
        assert_eq!(count_reached_summits(&map, 0, 2), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 3);
        assert_eq!(part2(&parse_input(TEST_INPUT)), 81);
    }
}
