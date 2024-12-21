use aoc_runner_derive::aoc;
use cached::proc_macro::cached;
use itertools::Itertools;
use lazy_static::lazy_static;
use pathfinding::prelude::astar_bag_collect;
use std::collections::HashMap;
use std::iter::zip;

type YX = (i32, i32);

type Keypad = HashMap<char, YX>;

lazy_static! {
    static ref NUMERICAL: Keypad = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        // (' ', (3, 0)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    static ref DIRECTIONAL: Keypad = HashMap::from([
        // (' ', (0, 0)),
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    static ref DIR_MAP: HashMap<YX, char> = HashMap::from([
        ((-1,0), '^'),
        ((1,0), 'v'),
        ((0,-1), '<'),
        ((0,1), '>'),
    ]);
}

fn routes_from_to(keypad: &Keypad, from: &YX, to: &YX) -> Vec<String> {
    let successors = |n: &YX| {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .map(|step| ((n.0 + step.0, n.1 + step.1), 1))
            .into_iter()
            .filter(|(next, _)| keypad.values().contains(&next))
    };
    let heuristic = |p: &YX| (to.0 - p.0).abs() + (to.1 - p.1).abs();
    let success = |p: &YX| p == to;

    let (new_moves, _length) = astar_bag_collect(from, successors, heuristic, success).unwrap();

    let routes = new_moves
        .iter()
        .map(|v| {
            let mut s: String = String::new();
            let mut prev = v.first().unwrap();
            for next in v.iter().skip(1) {
                let dir = (next.0 - prev.0, next.1 - prev.1);
                let char = DIR_MAP.get(&dir).unwrap();
                s.push(*char);
                prev = next;
            }
            s.push('A');
            s
        })
        .collect();

    routes
}

#[cached]
fn shortest_path(from: char, to: char, level: usize, max_level: usize) -> usize {
    if level == max_level - 1 {
        return 1;
    }

    let routes = if level == 0 {
        routes_from_to(
            &NUMERICAL,
            NUMERICAL.get(&from).unwrap(),
            NUMERICAL.get(&to).unwrap(),
        )
    } else {
        routes_from_to(
            &DIRECTIONAL,
            DIRECTIONAL.get(&from).unwrap(),
            DIRECTIONAL.get(&to).unwrap(),
        )
    };

    routes
        .iter()
        .map(|route| {
            let mut total = 0;
            let mut cur = 'A';
            for next in route.chars() {
                total += shortest_path(cur, next, level + 1, max_level);
                cur = next;
            }
            total
        })
        .min()
        .unwrap()
}

fn short_code(code: &str, levels: usize) -> usize {
    let mut from = 'A';
    let mut total = 0;
    for to in code.chars() {
        total += shortest_path(from, to, 0, levels);
        from = to;
    }

    total
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let codes = input.lines().collect_vec();

    let shortest_distances = codes.iter().map(|code| short_code(code, 4)).collect_vec();

    let numeric_parts = codes
        .iter()
        .map(|code| code[0..3].parse::<usize>().unwrap())
        .collect_vec();

    let complexities = zip(numeric_parts, shortest_distances)
        .map(|(num, len)| num * len)
        .collect_vec();

    complexities.iter().sum()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    let codes = input.lines().collect_vec();

    let shortest_distances = codes.iter().map(|code| short_code(code, 27)).collect_vec();

    let numeric_parts = codes
        .iter()
        .map(|code| code[0..3].parse::<usize>().unwrap())
        .collect_vec();

    let complexities = zip(numeric_parts, shortest_distances)
        .map(|(num, len)| num * len)
        .collect_vec();

    complexities.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "029A
980A
179A
456A
379A
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 126384);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 154115708116294);
    }
}
