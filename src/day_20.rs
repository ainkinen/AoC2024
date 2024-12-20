use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::bfs;
use std::collections::HashSet;

type YX = (i32, i32);

type Path = Vec<YX>;

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Path {
    let mut start = None;
    let mut end = None;

    let mut nodes: HashSet<YX> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    start = Some((y as i32, x as i32));
                    nodes.insert((y as i32, x as i32));
                }
                'E' => {
                    end = Some((y as i32, x as i32));
                    nodes.insert((y as i32, x as i32));
                }
                '.' => {
                    nodes.insert((y as i32, x as i32));
                }
                _ => {}
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let get_steps = |p: &YX| -> Vec<YX> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|dir| (p.0 + dir.0, p.1 + dir.1))
            .filter(|p| nodes.contains(p))
            .collect()
    };

    // Using pathfinding::bfs since the initial path is not really the challenge
    let route = bfs(&start, get_steps, |p| p == &end);

    route.unwrap()
}

fn dis(a: &YX, b: &YX) -> usize {
    (a.0 - b.0).unsigned_abs() as usize + (a.1 - b.1).unsigned_abs() as usize
}

fn solver(path: &Path, min_steps_saved: usize, max_shortcut_length: usize) -> usize {
    if path.is_empty() || min_steps_saved > (path.len() - 1) {
        panic!("Impossible to save this many steps")
    }

    let mut shortcuts = 0;

    let min_jump_size = min_steps_saved + 2;

    for (i, start) in path
        .iter()
        .take(path.len() - min_jump_size) // can't save enough steps after this point
        .enumerate()
    {
        let possible_shortcut_range = (i + min_jump_size)..path.len();
        let shortcut_destinations = possible_shortcut_range
            .map(|to_idx| (to_idx, dis(start, &path[to_idx])))
            .filter(|(_to_idx, d)| *d <= max_shortcut_length);

        for (to_id, d) in shortcut_destinations {
            let steps_saved = to_id - i - d;

            if steps_saved >= min_steps_saved {
                shortcuts += 1;
            }
        }
    }

    shortcuts
}

#[aoc(day20, part1)]
fn part1(path: &Path) -> usize {
    solver(path, 100, 2)
}

#[aoc(day20, part2)]
fn part2(path: &Path) -> usize {
    solver(path, 100, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_part1() {
        assert_eq!(solver(&parse_input(TEST_INPUT), 10, 2), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solver(&parse_input(TEST_INPUT), 70, 20), 41);
    }
}
