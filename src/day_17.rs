use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::zip;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Computer {
    let mut numbers = NUMBER
        .captures_iter(input)
        .map(|c| c.get(0).unwrap().as_str().parse::<u64>().unwrap());

    Computer {
        a: numbers.next().unwrap(),
        b: numbers.next().unwrap(),
        c: numbers.next().unwrap(),
        prog: numbers.collect(),

        counter: 0,
    }
}

#[derive(Debug, Clone, Default)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,

    prog: Vec<u64>,
    counter: u64,
}

impl Computer {
    fn combo(&self, n: &u64) -> u64 {
        match n {
            0..=3 => *n,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn next_output(&mut self) -> Option<u64> {
        while let (Some(op), Some(operand)) = (
            self.prog.get(self.counter as usize),
            self.prog.get(self.counter as usize + 1),
        ) {
            let mut out = None;
            match (op, operand) {
                (0, _) => {
                    // adv
                    self.a >>= self.combo(operand);
                }
                (1, _) => {
                    // bxl
                    self.b ^= operand
                }
                (2, _) => {
                    // bst
                    self.b = self.combo(operand) & 0b111;
                }
                (3, _) => {
                    // jnz
                    if self.a != 0 {
                        self.counter = *operand;
                        continue;
                    }
                }
                (4, _) => {
                    // bxc
                    self.b ^= self.c;
                }
                (5, _) => {
                    //out
                    out = Some(self.combo(operand) & 0b111);
                }
                (6, _) => {
                    // bdv
                    self.b = self.a >> self.combo(operand);
                }
                (7, _) => {
                    // cdv
                    self.c = self.a >> self.combo(operand);
                }
                _ => panic!("Unknown op {}", op),
            }

            self.counter += 2;
            if out.is_some() {
                return out;
            };
        }

        None
    }

    fn get_output(&mut self) -> Vec<u64> {
        std::iter::from_fn(|| self.next_output()).collect()
    }
}

#[aoc(day17, part1)]
fn part1(state: &Computer) -> String {
    let mut computer = state.clone();

    let output = computer.get_output();

    output.iter().map(|v| v.to_string()).join(",")
}

#[allow(dead_code)]
fn part2_brute(orig_computer: &Computer) -> u64 {
    for needle in 0.. {
        let mut computer = orig_computer.clone();
        computer.a = needle;

        let output = computer.get_output();
        if output == orig_computer.prog {
            return needle;
        }
    }
    panic!("No solution found!");
}

#[aoc(day17, part2)]
fn part2(orig_computer: &Computer) -> u64 {
    // Analyzing the code shows that the loop works over reg_a in 3 bit chunks.
    // Every chunk is used to generate one output, but previous numbers also affect the output.

    let mut needle = 0u64;
    'outer: for _ in 0..orig_computer.prog.len() {
        for chunk in 0.. {
            let mut computer = Computer {
                a: needle + chunk,
                ..orig_computer.clone()
            };

            let orig_prog = orig_computer.prog.clone();
            let output = computer.get_output();

            if zip(orig_prog.iter().rev(), output.iter().rev()).all(|(a, b)| a == b) {
                // end matches
                if output.len() == orig_prog.len() {
                    return needle + chunk;
                }

                needle = (needle + chunk) << 3;

                continue 'outer;
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    static TEST_INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_run() {
        let mut computer = Computer {
            c: 9,
            prog: vec![2, 6],
            ..Computer::default()
        };
        computer.get_output();
        assert_eq!(computer.b, 1);

        let mut computer = Computer {
            a: 10,
            prog: vec![5, 0, 5, 1, 5, 4],
            ..Computer::default()
        };
        let output = computer.get_output();
        assert_eq!(output, vec![0, 1, 2]);

        let mut computer = Computer {
            a: 2024,
            prog: vec![0, 1, 5, 4, 3, 0],
            ..Computer::default()
        };
        let output = computer.get_output();
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.a, 0);

        let mut computer = Computer {
            b: 29,
            prog: vec![1, 7],
            ..Computer::default()
        };
        computer.get_output();
        assert_eq!(computer.b, 26);

        let mut computer = Computer {
            b: 2024,
            c: 43690,
            prog: vec![4, 0],
            ..Computer::default()
        };
        computer.get_output();
        assert_eq!(computer.b, 44354);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(TEST_INPUT)),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn test_part2_brute() {
        assert_eq!(part2_brute(&parse_input(TEST_INPUT_2)), 117440);
    }
}
