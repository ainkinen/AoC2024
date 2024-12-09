use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy)]
enum BlockAlloc {
    File { id: u64 },
    FreeSpace,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Alloc {
    id: Option<u64>,
    len: u64,
    moved: bool,
}

#[aoc_generator(day9, part1)]
fn parse_input(input: &str) -> Vec<BlockAlloc> {
    // Make sure that the len is even for tuple chunking
    let input = if input.len() % 2 == 0 {
        input.to_string()
    } else {
        String::from(input) + "0"
    };

    let x = input
        .chars()
        .tuples()
        .enumerate()
        .flat_map(|(i, (file_length_char, space_length_char))| {
            let file_length = file_length_char.to_digit(10).unwrap();
            let files = [BlockAlloc::File { id: i as u64 }].repeat(file_length as usize);
            let space_length = space_length_char.to_digit(10).unwrap();
            let spaces = [BlockAlloc::FreeSpace {}].repeat(space_length as usize);
            files.into_iter().chain(spaces)
        })
        .collect::<Vec<_>>();

    x
}

#[aoc_generator(day9, part2)]
fn parse_input_2(input: &str) -> Vec<Alloc> {
    // Make sure that the len is even for tuple chunking
    let input = if input.len() % 2 == 0 {
        input.to_string()
    } else {
        String::from(input) + "0"
    };

    let x = input
        .chars()
        .tuples()
        .enumerate()
        .flat_map(|(i, (file_length_char, space_length_char))| {
            let file_length = file_length_char.to_digit(10).unwrap();
            let space_length = space_length_char.to_digit(10).unwrap();

            [
                Alloc {
                    id: Some(i as u64),
                    len: file_length as u64,
                    moved: false,
                },
                Alloc {
                    id: None,
                    len: space_length as u64,
                    moved: false,
                },
            ]
        })
        .filter(|a| a.len > 0)
        .collect();

    x
}

fn first_free(v: &[BlockAlloc], from: usize) -> Option<usize> {
    if let Some(i) = v
        .iter()
        .skip(from)
        .position(|a| matches!(a, BlockAlloc::FreeSpace))
    {
        return Some(from + i);
    }
    None
}

fn last_non_free(v: &[BlockAlloc], from: usize) -> Option<usize> {
    for i in (0..from + 1).rev() {
        match v[i] {
            BlockAlloc::FreeSpace => continue,
            BlockAlloc::File { id: _ } => return Some(i),
        }
    }

    None
}

#[aoc(day9, part1)]
fn part1(input: &[BlockAlloc]) -> u64 {
    let mut disk = input.to_vec();

    let mut start = 0;
    let mut end = disk.len() - 1;

    while let (Some(new_start), Some(new_end)) =
        (first_free(&disk, start), last_non_free(&disk, end))
    {
        if new_start >= new_end {
            break;
        }

        disk.swap(new_start, new_end);

        start = new_start;
        end = new_end;
    }

    disk.iter().enumerate().fold(0u64, |acc, (i, &v)| match v {
        BlockAlloc::FreeSpace => acc,
        BlockAlloc::File { id } => acc + (i as u64 * id),
    })
}

fn find_available_spot(disk: &[Alloc], min_size: u64, max: usize) -> Option<usize> {
    disk.iter()
        .take(max)
        .position(|alloc| alloc.id.is_none() && alloc.len >= min_size)
}

fn next_file_to_move(disk: &[Alloc]) -> Option<usize> {
    disk.iter().rposition(|alloc| !alloc.moved)
}

#[aoc(day9, part2)]
fn part2(input: &[Alloc]) -> u64 {
    let mut disk = input.to_vec();

    while let Some(i_to_move) = next_file_to_move(&disk) {
        disk[i_to_move].moved = true;

        let to_move_copy = &disk[i_to_move];
        if let Some(i_free_space) = find_available_spot(&disk, to_move_copy.len, i_to_move) {
            if disk[i_to_move].len == disk[i_free_space].len {
                disk.swap(i_to_move, i_free_space);
            } else {
                disk[i_free_space].len -= disk[i_to_move].len;
                disk.insert(
                    i_free_space,
                    Alloc {
                        id: None,
                        len: disk[i_to_move].len,
                        moved: true,
                    },
                );

                disk.swap(i_to_move + 1, i_free_space);
            }
        }

        // compact the end
        while let Some(Alloc { id: None, .. }) = disk.last() {
            let _ = disk.pop();
        }
    }

    disk.iter()
        .flat_map(|alloc| {
            if alloc.id.is_none() {
                vec![0; alloc.len as usize]
            } else {
                vec![alloc.id.unwrap(); alloc.len as usize]
            }
        })
        .enumerate()
        .map(|(i, id)| i as u64 * id)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input_2(TEST_INPUT)), 2858);
    }
}
