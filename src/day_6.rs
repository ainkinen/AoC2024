use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Coord = (i32, i32);

#[derive(Debug)]
struct Input {
    obstacles: HashSet<Coord>,
    starting_point: Coord,
    side: i32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn step(p: &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::UP => (p.0 - 1, p.1),
        Direction::RIGHT => (p.0, p.1 + 1),
        Direction::DOWN => (p.0 + 1, p.1),
        Direction::LEFT => (p.0, p.1 - 1),
    }
}

fn turn(dir: &Direction) -> Direction {
    match dir {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

fn in_bounds(p: &Coord, side: i32) -> bool {
    (0..side).contains(&p.0) && (0..side).contains(&p.1)
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    let mut obstacles = HashSet::new();
    let mut starting_point: Coord = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    let _ = obstacles.insert((y as i32, x as i32));
                }
                '^' => starting_point = (y as i32, x as i32),
                _ => {}
            }
        }
    }

    Input {
        obstacles,
        starting_point,
        side: input.lines().count() as i32,
    }
}

#[allow(dead_code)]
fn print_visited(input: &Input, visited: &HashSet<Coord>) {
    for y in 0..input.side {
        for x in 0..input.side {
            let p = (y, x);
            let c = match p {
                p if p == input.starting_point => '^',
                p if input.obstacles.contains(&p) => '#',
                p if visited.contains(&p) => 'X',
                _ => '.',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn get_path(input: &Input) -> HashSet<Coord> {
    let mut guard_at = input.starting_point;
    let mut guard_direction = Direction::UP;

    let mut visited: HashSet<Coord> = HashSet::new();

    loop {
        visited.insert(guard_at);

        let next = step(&guard_at, &guard_direction);
        if !in_bounds(&next, input.side) {
            // guard stepped out
            return visited;
        }

        if input.obstacles.contains(&next) {
            // ran into obstacle
            guard_direction = turn(&guard_direction);
        } else {
            // continue forward
            guard_at = next;
        }
    }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    let visited = get_path(input);
    // print_visited(input, &visited);
    visited.len()
}

fn does_loop(starting_point: &Coord, obstacles: &HashSet<Coord>, side: i32) -> bool {
    let mut guard_at = starting_point.clone();
    let mut guard_direction = Direction::UP;

    let mut visited: HashSet<(Coord, Direction)> = HashSet::new();

    loop {
        let new_step = visited.insert((guard_at, guard_direction));
        if !new_step {
            // Step already found on the path, entering a loop
            return true;
        }

        let next = step(&guard_at, &guard_direction);
        if !in_bounds(&next, side) {
            // guard stepped out without entering a loop
            return false;
        }

        if obstacles.contains(&next) {
            // ran into obstacle
            guard_direction = turn(&guard_direction);
        } else {
            // continue forward
            guard_at = next;
        }
    }
}

// #[aoc(day6, part2, slow)]
#[allow(dead_code)]
fn part2(input: &Input) -> i32 {
    let mut loop_causing_obstacles: HashSet<Coord> = HashSet::new();

    for y in 0..input.side {
        for x in 0..input.side {
            let new_obstacle = (y, x);

            if input.obstacles.contains(&new_obstacle) || new_obstacle == input.starting_point {
                // not a valid location
                continue;
            }

            let mutated_obstacles = {
                let mut set = input.obstacles.clone();
                set.insert(new_obstacle);
                set
            };

            if does_loop(&input.starting_point, &mutated_obstacles, input.side) {
                loop_causing_obstacles.insert(new_obstacle);
            }
        }
    }

    loop_causing_obstacles.len() as i32
}

#[aoc(day6, part2, faster)]
fn part2_faster(input: &Input) -> i32 {
    let mut loop_causing_obstacles: HashSet<Coord> = HashSet::new();

    // The added obstacle must be along the original path. Otherwise, the guard would not hit it.
    let orig_path = get_path(input);
    for p in orig_path {
        let new_obstacle = p;

        if input.obstacles.contains(&new_obstacle) || new_obstacle == input.starting_point {
            // not a valid location
            continue;
        }

        let mutated_obstacles = {
            let mut set = input.obstacles.clone();
            set.insert(new_obstacle);
            set
        };

        if does_loop(&input.starting_point, &mutated_obstacles, input.side) {
            loop_causing_obstacles.insert(new_obstacle);
        }
    }
    loop_causing_obstacles.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 6);
    }

    #[test]
    fn test_part2_faster() {
        assert_eq!(part2_faster(&parse_input(TEST_INPUT)), 6);
    }
}
