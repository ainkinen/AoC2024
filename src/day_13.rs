use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

type XY = (i64, i64);

type Machine = (XY, XY, XY);
type Machines = Vec<Machine>;

#[aoc_generator(day13, part1)]
fn parse_input_1(input: &str) -> Machines {
    let pattern = Regex::new(r"(\d+)").unwrap();

    let mut machines = vec![];

    let numbers: Vec<i64> = pattern
        .captures_iter(input)
        .map(|c| c[1].parse().unwrap())
        .collect();

    for chunk in numbers.chunks(6) {
        let [ax, ay, bx, by, px, py] = chunk.try_into().unwrap();

        machines.push(((ax, ay), (bx, by), (px, py)));
    }

    machines
}

#[aoc_generator(day13, part2)]
fn parse_input_2(input: &str) -> Machines {
    let machines = parse_input_1(input);

    machines
        .iter()
        .map(|&(a, b, p)| (a, b, (10000000000000 + p.0, 10000000000000 + p.1)))
        .collect()
}

fn solve(a: XY, b: XY, p: XY) -> Option<(f64, f64)> {
    let v1 = Vector2::new(a.0 as f64, a.1 as f64);
    let v2 = Vector2::new(b.0 as f64, b.1 as f64);
    let p = Vector2::new(p.0 as f64, p.1 as f64);

    let a = Matrix2::from_columns(&[v1, v2]);

    if a.determinant().abs() < 1e-10 {
        // The vectors are collinear. The matrix cannot be inverted.
        return None;
    }

    let a_inv = a.try_inverse().unwrap();
    let multipliers = a_inv * p;
    Some((multipliers[0], multipliers[1]))
}

fn brute_solve(a: XY, b: XY, p: XY) -> Option<XY> {
    let mut solutions = vec![];

    for count_a in 0..100 {
        for count_b in 0..100 {
            if (count_a * a.0 + count_b * b.0) == p.0 && (count_a * a.1 + count_b * b.1) == p.1 {
                solutions.push((count_a, count_b));
            }
        }
    }

    solutions
        .into_iter()
        .min_by(|&left, &right| (left.0 * 3 + left.1).cmp(&(right.0 * 3 + right.1)))
}

fn close_enough(num: f64) -> Option<i64> {
    if (num.round() - num).abs() < 0.001 {
        Some(num.round() as i64)
    } else {
        None
    }
}

#[aoc(day13, part1, brute)]
fn part1_brute(machines: &Machines) -> i64 {
    machines
        .iter()
        .flat_map(|m| brute_solve(m.0, m.1, m.2))
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn solve_parts(machines: &Machines) -> i64 {
    machines
        .iter()
        .flat_map(|m| solve(m.0, m.1, m.2))
        .map(|(a, b)| {
            if let (Some(a_count), Some(b_count)) = (close_enough(a), close_enough(b)) {
                a_count * 3 + b_count
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day13, part1)]
fn part1(machines: &Machines) -> i64 {
    solve_parts(machines)
}

#[aoc(day13, part2)]
fn part2(machines: &Machines) -> i64 {
    solve_parts(machines)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input_1(TEST_INPUT)), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input_2(TEST_INPUT)), 875318608908);
    }
}
