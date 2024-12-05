use aoc_runner_derive::{aoc, aoc_generator};

struct Grid {
    len_y: usize,
    len_x: usize,
    data: Vec<Vec<char>>,
}
type YX = (i32, i32);
type Coords = [YX; 4];

fn get_xmas_coords(p: &YX) -> Vec<Coords> {
    let deltas: Vec<YX> = vec![
        (0, 1),   // →
        (1, 1),   // ↘
        (1, 0),   // ↓
        (1, -1),  // ↙
        (0, -1),  // ←
        (-1, -1), // ↖
        (-1, 0),  // ↑
        (-1, 1),  // ↗
    ];

    deltas
        .iter()
        .map(|d| {
            [
                (p.0, p.1),
                (p.0 + d.0, p.1 + d.1),
                (p.0 + 2 * d.0, p.1 + 2 * d.1),
                (p.0 + 3 * d.0, p.1 + 3 * d.1),
            ]
        })
        .collect()
}

fn get_corner_coords(p: &YX) -> Vec<Coords> {
    let (y, x) = p;
    vec![
        // Coords of expected M M S S
        [
            (y - 1, x - 1), // ↓
            (y - 1, x + 1), // M M
            (y + 1, x - 1), //  A
            (y + 1, x + 1), // S S
        ],
        [
            (y - 1, x - 1), // →
            (y + 1, x - 1), // M S
            (y - 1, x + 1), //  A
            (y + 1, x + 1), // M S
        ],
        [
            (y + 1, x - 1), // ↑
            (y + 1, x + 1), // S S
            (y - 1, x - 1), //  A
            (y - 1, x + 1), // M M
        ],
        [
            (y - 1, x + 1), // ←
            (y + 1, x + 1), // S M
            (y - 1, x - 1), //  A
            (y + 1, x - 1), // S M
        ],
    ]
}

fn get_grid_char(grid: &Grid, p: &YX) -> Option<char> {
    if let Some(line) = grid.data.get(p.0 as usize) {
        return line.get(p.1 as usize).cloned();
    }

    None
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Grid {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    Grid {
        len_y: data.len(),
        len_x: data[0].len(),
        data,
    }
}

#[aoc(day4, part1)]
fn part1(grid: &Grid) -> i32 {
    let mut count = 0;
    for y in 0..grid.len_y {
        for x in 0..grid.len_x {
            let p = (y as i32, x as i32);
            if get_grid_char(grid, &p) != Some('X') {
                continue;
            }
            for coord in get_xmas_coords(&p) {
                if let (Some(m), Some(a), Some(s)) = (
                    get_grid_char(grid, &coord[1]),
                    get_grid_char(grid, &coord[2]),
                    get_grid_char(grid, &coord[3]),
                ) {
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
fn part2(grid: &Grid) -> i32 {
    let mut count = 0;
    for y in 0..grid.len_y {
        for x in 0..grid.len_x {
            let p = (y as i32, x as i32);
            match get_grid_char(grid, &p) {
                Some('A') => {
                    let rotations = get_corner_coords(&p);
                    if rotations.iter().any(|coord| {
                        get_grid_char(grid, &coord[0]) == Some('M')
                            && get_grid_char(grid, &coord[1]) == Some('M')
                            && get_grid_char(grid, &coord[2]) == Some('S')
                            && get_grid_char(grid, &&coord[3]) == Some('S')
                    }) {
                        count += 1
                    }
                }
                _ => continue,
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_get_xmas_coords() {
        let c = get_xmas_coords(&(0, 0));
        assert_eq!(c.len(), 8);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 9);
    }
}
