use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

type YX = (i32, i32);
#[derive(Debug)]
struct Input {
    // nodes: HashSet<YX>,
    edges: HashMap<YX, Vec<(YX, char)>>, // from-to
    start: YX,
    end: YX,
}

type Path = Vec<(YX, char)>;

lazy_static! {
    static ref DIRS: Vec<YX> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    static ref DIR_CHARS: HashMap<YX, char> =
        HashMap::from([((0, 1), '>'), ((0, -1), '<'), ((1, 0), 'v'), ((-1, 0), '^')]);
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Input {
    let mut start = None;
    let mut end = None;

    let mut nodes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let from = (y as i32, x as i32);

            match ch {
                'S' => {
                    start = Some(from);
                    nodes.insert(from);
                }
                'E' => {
                    end = Some(from);
                    nodes.insert(from);
                }
                '.' => {
                    nodes.insert(from);
                }
                _ => {}
            }
        }
    }

    let mut edges: HashMap<YX, Vec<(YX, char)>> = HashMap::new();
    for node in &nodes {
        let (y, x) = node;

        for dir in DIRS.iter() {
            let neighbor = (y + dir.0, x + dir.1);
            if nodes.contains(&neighbor) {
                edges
                    .entry(*node)
                    .or_default()
                    .push((neighbor, DIR_CHARS[dir]));
            }
        }
    }
    Input {
        // nodes,
        edges,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn get_paths(input: &Input) -> Vec<Path> {
    let mut paths: Vec<Path> = vec![vec![(input.start, '>')]];

    let mut completed_paths: Vec<Path> = vec![];

    let mut loc_scores: HashMap<(YX, char), i32> = HashMap::new();

    while let Some(path) = paths.pop() {
        let &(head, _head_dir) = path.last().unwrap();
        if head == input.end {
            completed_paths.push(path);
            continue;
        }

        for neighbor in &input.edges[&head] {
            // if path.contains(neighbor) {
            //     // Loop
            //     continue;
            // }

            let mut new_path = path.clone();
            new_path.push(*neighbor);

            let score = score_path(&new_path);

            if score <= *loc_scores.get(neighbor).unwrap_or(&i32::MAX) {
                // Best score for this loc so far
                loc_scores.insert(*neighbor, score);
                paths.push(new_path);
            }
        }
    }

    completed_paths
}

fn score_path(path: &Path) -> i32 {
    let (_start, mut dir) = path.first().unwrap();

    let mut total = 0;

    for &(_next_loc, next_dir) in path.iter().skip(1) {
        total += 1;
        if next_dir != dir {
            total += 1000;
        }
        dir = next_dir;
    }

    total
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> i32 {
    let paths = get_paths(input);
    paths.iter().map(score_path).min().unwrap()
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    let paths = get_paths(input);
    let min_length = paths.iter().map(score_path).min().unwrap();

    let min_paths = paths.iter().filter(|p| score_path(p) == min_length);

    let min_path_coords = min_paths
        .flat_map(|p| p.iter().map(|(coord, _dir)| coord))
        .collect::<HashSet<&YX>>();

    min_path_coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    static TEST_INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 7036);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 45);
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 64);
    }
}
