use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"-*?\d+").unwrap();
}

type XY = (i32, i32);

#[derive(Debug, Clone)]
struct Robot {
    loc: XY,
    vel: XY,
}
type Robots = Vec<Robot>;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Robots {
    let mut robots = Vec::new();

    let numbers: Vec<i32> = NUMBER
        .captures_iter(input)
        .map(|cap| cap[0].parse::<i32>().unwrap())
        .collect();

    for chunk in numbers.chunks_exact(4) {
        let [px, py, vx, vy] = chunk.try_into().unwrap();

        robots.push(Robot {
            loc: (px, py),
            vel: (vx, vy),
        });
    }

    robots
}

fn step_robots(robots: &Robots, len_x: i32, len_y: i32, steps: i32) -> Robots {
    let mut robots: Vec<Robot> = robots.clone();

    robots.iter_mut().for_each(|r| {
        r.loc = (
            (((r.loc.0 + steps * r.vel.0) % len_x) + len_x) % len_x,
            (((r.loc.1 + steps * r.vel.1) % len_y) + len_y) % len_y,
        );
    });

    robots
}

fn part1_solve(robots: &Robots, len_x: i32, len_y: i32, steps: i32) -> usize {
    let robots = step_robots(robots, len_x, len_y, steps);

    let (mut nw, mut ne, mut sw, mut se) = (0, 0, 0, 0);
    let w_x = 0..len_x / 2;
    let e_x = (len_x / 2) + 1..len_x;
    let n_y = 0..len_y / 2;
    let s_y = (len_y / 2) + 1..len_y;

    for loc in robots.iter().map(|r| r.loc) {
        match loc {
            (x, y) if w_x.contains(&x) && n_y.contains(&y) => nw += 1,
            (x, y) if e_x.contains(&x) && n_y.contains(&y) => ne += 1,
            (x, y) if e_x.contains(&x) && s_y.contains(&y) => se += 1,
            (x, y) if w_x.contains(&x) && s_y.contains(&y) => sw += 1,
            _ => {
                // In the middle. Noop.
            }
        }
    }

    nw * ne * se * sw
}

#[aoc(day14, part1)]
fn part1(robots: &Robots) -> usize {
    part1_solve(robots, 101, 103, 100)
}

#[allow(dead_code)]
fn graph_robots(robots: &Robots, len_x: i32, len_y: i32) {
    let locs: HashSet<XY> = robots.iter().map(|r| r.loc).collect();

    for y in 0..len_y {
        for x in 0..len_x {
            let char = if locs.contains(&(x, y)) { 'X' } else { '.' };
            print!("{}", char);
        }
        println!();
    }
}

fn mean(data: &[XY]) -> Option<(f64, f64)> {
    if data.is_empty() {
        return None;
    }

    let len = data.len() as f64;

    let mean_x = data.iter().map(|&(x, _)| x as f64).sum::<f64>() / (len);
    let mean_y = data.iter().map(|&(_, y)| y as f64).sum::<f64>() / (len);

    Some((mean_x, mean_y))
}

fn std_deviation(data: &[XY]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }

    let mean_point = mean(data)?;

    let variance = data
        .iter()
        .map(|&(x, y)| {
            // Compute Euclidean distance from the mean point
            let dx = x as f64 - mean_point.0;
            let dy = y as f64 - mean_point.1;
            dx * dx + dy * dy
        })
        .sum::<f64>()
        / data.len() as f64;

    Some(variance.sqrt())
}

fn find_possible_image(robots: &Robots, len_x: i32, len_y: i32) -> usize {
    // Image should have some structure
    // Find the step with smallest std deviation

    let deviations = (0..10000)
        .map(|steps| {
            let robots = step_robots(robots, len_x, len_y, steps);
            let locs: Vec<XY> = robots.iter().map(|r| r.loc).collect();
            std_deviation(&locs).unwrap()
        })
        .collect_vec();

    deviations
        .iter()
        .position_min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

#[aoc(day14, part2)]
fn part2(robots: &Robots) -> i32 {
    let len_x = 101;
    let len_y = 103;

    let maybe_img_steps = find_possible_image(robots, len_x, len_y) as i32;

    // let robots_in_image = step_robots(robots, len_x, len_y, maybe_img_steps);
    // graph_robots(&robots_in_image, len_x, len_y);

    #[allow(clippy::let_and_return)]
    maybe_img_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1_solve(&parse_input(TEST_INPUT), 11, 7, 100), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_possible_image(&parse_input(TEST_INPUT), 11, 7), 24);
    }
}
