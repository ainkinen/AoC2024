use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

type Coord = (i32, i32);
type AntennaGroups = HashMap<char, HashSet<Coord>>;

struct Input {
    antenna_groups: AntennaGroups,
    range: Range<i32>,
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Input {
    let mut len_y = 0;
    let mut antennae = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        len_y += 1;
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let _ = antennae
                .entry(char)
                .or_insert(HashSet::<Coord>::new())
                .insert((y as i32, x as i32));
        }
    }

    Input {
        antenna_groups: antennae,
        range: 0..len_y,
    }
}

fn get_anti_nodes(antennae: &HashSet<Coord>, range: &Range<i32>) -> HashSet<Coord> {
    let pairs = antennae.iter().tuple_combinations();

    pairs
        .flat_map(|(a, b)| {
            let dy = a.0 - b.0;
            let dx = a.1 - b.1;

            vec![(a.0 + dy, a.1 + dx), (b.0 - dy, b.1 - dx)]
        })
        .filter(|p| range.contains(&p.0) && range.contains(&p.1))
        .collect()
}

fn get_repeating_anti_nodes(antennae: &HashSet<Coord>, range: &Range<i32>) -> HashSet<Coord> {
    let pairs = antennae.iter().tuple_combinations();

    let in_range = |(y, x)| range.contains(&y) && range.contains(&x);

    pairs
        .flat_map(|(a, b)| {
            let dy = a.0 - b.0;
            let dx = a.1 - b.1;

            let mut coords = vec![];

            for m in 0.. {
                let p = (a.0 + m * dy, a.1 + m * dx);
                if !in_range(p) {
                    break;
                }
                coords.push(p);
            }

            for m in 0.. {
                let p = (a.0 - m * dy, a.1 - m * dx);
                if !in_range(p) {
                    break;
                }
                coords.push(p);
            }

            coords
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> i32 {
    let anti_nodes = input
        .antenna_groups
        .values()
        .flat_map(|antennae| get_anti_nodes(antennae, &input.range))
        .collect::<HashSet<Coord>>();

    anti_nodes.len() as i32
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> i32 {
    let possible_anti_nodes = input
        .antenna_groups
        .values()
        .flat_map(|antennae| get_repeating_anti_nodes(antennae, &input.range))
        .collect::<HashSet<Coord>>();

    possible_anti_nodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    static TEST_INPUT_2: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(TEST_INPUT);

        assert_eq!(
            parsed.antenna_groups,
            HashMap::from([
                ('A', HashSet::from([(5, 6), (8, 8), (9, 9)])),
                ('0', HashSet::from([(1, 8), (2, 5), (3, 7), (4, 4)])),
            ])
        );
        assert_eq!(parsed.range, 0..12);
    }

    #[test]
    fn test_get_anti_nodes() {
        let antennae1 = HashSet::from([(1, 1), (2, 2)]);
        let range = 0..4;
        assert_eq!(
            get_anti_nodes(&antennae1, &range),
            HashSet::from([(0, 0), (3, 3)])
        );

        let antennae2 = HashSet::from([(1, 2), (2, 1)]);
        assert_eq!(
            get_anti_nodes(&antennae2, &range),
            HashSet::from([(0, 3), (3, 0)])
        );
    }

    #[test]
    fn test_get_repeating_anti_nodes() {
        let antennae = HashSet::from([(0, 0), (1, 3), (2, 1)]);
        let range = 0..10;
        let result = get_repeating_anti_nodes(&antennae, &range);
        assert_eq!(
            result,
            HashSet::from([
                (0, 0),
                (0, 5),
                (1, 3),
                (2, 1),
                (2, 6),
                (3, 9),
                (4, 2),
                (6, 3),
                (8, 4)
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 9);
        assert_eq!(part2(&parse_input(TEST_INPUT)), 34);
    }
}
