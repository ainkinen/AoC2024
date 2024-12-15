use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

type YX = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    map_y: i32,
    map_x: i32,
    map: HashMap<YX, char>,
    moves: Vec<Dir>,
    robot_at: YX,
}

#[aoc_generator(day15, part1)]
fn parse_input(input: &str) -> Input {
    let (map_str, moves_str) = input.split("\n\n").collect_tuple().unwrap();

    let mut map = HashMap::new();
    let mut robot_at = None;

    for (y, line) in map_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = (y as i32, x as i32);
            match c {
                '#' | 'O' => {
                    map.insert(coord, c);
                }
                '@' => robot_at = Some(coord),
                _ => continue,
            }
        }
    }

    let mut moves = Vec::new();
    for c in moves_str.chars() {
        match c {
            '^' => moves.push(Dir::Up),
            '>' => moves.push(Dir::Right),
            'v' => moves.push(Dir::Down),
            '<' => moves.push(Dir::Left),
            '\n' => continue,
            _ => panic!("Unknown move {}", c),
        }
    }

    Input {
        map_y: map_str.lines().count() as i32,
        map_x: map_str.lines().next().unwrap().len() as i32,
        map,
        moves,
        robot_at: robot_at.unwrap(),
    }
}

fn double_map(map: &HashMap<YX, char>) -> HashMap<YX, char> {
    let new_map = map
        .iter()
        .flat_map(|(&(y, x), c)| match c {
            '#' => [((y, x * 2), '#'), ((y, x * 2 + 1), '#')],
            'O' => [((y, x * 2), '['), ((y, x * 2 + 1), ']')],
            _ => unreachable!("Unknown map tile: {:?}", y),
        })
        .collect();

    new_map
}

#[aoc_generator(day15, part2)]
fn parse_input_2(input: &str) -> Input {
    let parsed = parse_input(input);

    Input {
        map: double_map(&parsed.map),
        map_x: parsed.map_x * 2,
        robot_at: (parsed.robot_at.0, parsed.robot_at.1 * 2),
        ..parsed
    }
}

fn next(p: &YX, dir: &Dir) -> YX {
    match dir {
        Dir::Up => (p.0 - 1, p.1),
        Dir::Down => (p.0 + 1, p.1),
        Dir::Left => (p.0, p.1 - 1),
        Dir::Right => (p.0, p.1 + 1),
    }
}

fn step_simple(map: &mut HashMap<YX, char>, robot_at: &YX, dir: &Dir) -> YX {
    let chain_to_move = chain_to_move(map, robot_at, dir);

    match chain_to_move {
        None => {
            // Can't move. Robot stays put.
            *robot_at
        }
        Some(mut chain) => {
            let in_front_of_robot = next(robot_at, dir);
            if !chain.is_empty() {
                let in_front_of_last = next(chain.last().unwrap(), dir);
                chain.push(in_front_of_last);
                for (front, behind) in chain.iter().rev().tuple_windows() {
                    map.insert(*front, map[behind]);
                }
                map.remove(&in_front_of_robot);
            }

            in_front_of_robot
        }
    }
}

fn chain_to_move(map: &mut HashMap<YX, char>, robot_at: &YX, dir: &Dir) -> Option<Vec<YX>> {
    let mut chain = vec![];

    let mut head = next(robot_at, dir);
    while let Some(char) = map.get(&head) {
        match char {
            '#' => return None,
            _ => {
                chain.push(head);
                head = next(&head, dir);
            }
        }
    }

    Some(chain)
}

fn step_complicated(map: &mut HashMap<YX, char>, robot_at: &YX, dir: &Dir) -> YX {
    let to_move = items_to_move(map, robot_at, dir);

    match to_move {
        None => {
            // Can't move
            *robot_at
        }
        Some(to_move) => {
            let new_robot = next(robot_at, dir);

            let mut to_move = to_move.iter().collect_vec();

            // Sort opposite to dir to move boxes from front to back
            match dir {
                Dir::Up => {
                    to_move.sort(); // from up to down
                }

                Dir::Down => {
                    to_move.sort(); // from up to down
                    to_move.reverse(); // from down to up
                }
                _ => unreachable!("Should not be used with left-right"),
            }

            // Move boxes. No overwrites because of sorting.
            for coord in to_move {
                let char = map.remove(coord).unwrap();
                map.insert(next(coord, dir), char);
            }

            new_robot
        }
    }
}

fn items_to_move(map: &mut HashMap<YX, char>, robot_at: &YX, dir: &Dir) -> Option<HashSet<YX>> {
    let mut coords_to_move = HashSet::new();
    let mut todo = vec![*robot_at];

    while let Some(coord) = todo.pop() {
        let next = next(&coord, dir);

        if let Some(tile) = map.get(&next) {
            match tile {
                '#' => {
                    // Hitting wall. Can't move anything.
                    return None;
                }
                '[' => {
                    for b in [next, (next.0, next.1 + 1)] {
                        if coords_to_move.insert(b) {
                            todo.push(b);
                        }
                    }
                }
                ']' => {
                    for b in [next, (next.0, next.1 - 1)] {
                        if coords_to_move.insert(b) {
                            todo.push(b);
                        }
                    }
                }
                _ => unreachable!("Should not be used with left-right"),
            }
        }
    }

    Some(coords_to_move)
}

#[allow(dead_code)]
fn graph(input: &Input, map: &HashMap<YX, char>, robot_at: &YX) {
    for y in 0..input.map_y {
        for x in 0..input.map_x {
            let coord = (y, x);
            if *robot_at == coord {
                print!("@");
            } else if let Some(char) = map.get(&coord) {
                print!("{}", char);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn score_box(b: &YX) -> i32 {
    b.0 * 100 + b.1
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> i32 {
    let mut map = input.map.clone();

    let mut robot_at = input.robot_at;

    // println!("{:?}", "Initial state");
    // graph(input, &map, &robot_at);

    for dir in input.moves.iter() {
        robot_at = step_simple(&mut map, &robot_at, dir);

        // println!("{:?}", dir);
        // graph(input, &map, &robot_at);
    }

    map.iter()
        .filter_map(|(coord, v)| if *v == 'O' { Some(coord) } else { None })
        .map(score_box)
        .sum()
}

#[aoc(day15, part2)]
fn part2(input: &Input) -> i32 {
    let mut map = input.map.clone();

    let mut robot_at = input.robot_at;

    // println!("{:?}", "Initial state");
    // graph(input, &map, &robot_at);

    for dir in input.moves.iter() {
        match dir {
            Dir::Left | Dir::Right => robot_at = step_simple(&mut map, &robot_at, dir),
            Dir::Up | Dir::Down => robot_at = step_complicated(&mut map, &robot_at, dir),
        }

        // println!("{:?}", dir);
        // graph(input, &map, &robot_at);
    }

    map.iter()
        .filter_map(|(coord, v)| if *v == '[' { Some(coord) } else { None })
        .map(score_box)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    static TEST_INPUT_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    static TEST_INPUT_SMALL_2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_SMALL)), 2028);
        assert_eq!(part1(&parse_input(TEST_INPUT_LARGE)), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input_2(TEST_INPUT_SMALL_2)), 105 + 207 + 306);
        assert_eq!(part2(&parse_input_2(TEST_INPUT_LARGE)), 9021);
    }
}
