use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Node = (i32, i32, char); // Y,X, id
type Nodes = HashSet<Node>;
type Edges = HashMap<Node, Vec<Node>>; // from -> to

type Corners = HashMap<Node, usize>;

#[derive(Debug)]
struct Input {
    nodes: Nodes,
    edges: Edges,
    corners: Corners,
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Input {
    let mut nodes: Nodes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            nodes.insert((y as i32, x as i32, c));
        }
    }

    let mut edges: Edges = HashMap::new();
    let mut corners: Corners = HashMap::new();

    for from_node in &nodes {
        let &(y, x, id) = from_node;

        let [n, ne, e, se, s, sw, w, nw] = [
            (y - 1, x),
            (y - 1, x + 1),
            (y, x + 1),
            (y + 1, x + 1),
            (y + 1, x),
            (y + 1, x - 1),
            (y, x - 1),
            (y - 1, x - 1),
        ]
        .iter()
        .map(|&(y, x)| nodes.get(&(y, x, id)))
        .collect_vec()
        .try_into()
        .unwrap();

        let neighbors = [n, s, w, e];

        neighbors.iter().flatten().for_each(|to_node| {
            edges.entry(*from_node).or_default().push(**to_node);
        });

        // Convex corners
        if let (None, None) = (n, w) {
            *corners.entry(*from_node).or_default() += 1;
        }
        if let (None, None) = (neighbors[0], neighbors[3]) {
            *corners.entry(*from_node).or_default() += 1;
        }
        if let (None, None) = (neighbors[3], neighbors[1]) {
            *corners.entry(*from_node).or_default() += 1;
        }
        if let (None, None) = (neighbors[1], neighbors[2]) {
            *corners.entry(*from_node).or_default() += 1;
        }

        // Concave corners
        if let (Some(_), Some(_), None) = (n, e, ne) {
            *corners.entry(*from_node).or_default() += 1;
        }
        if let (Some(_), Some(_), None) = (e, s, se) {
            *corners.entry(*from_node).or_default() += 1;
        }
        if let (Some(_), Some(_), None) = (s, w, sw) {
            *corners.entry(*from_node).or_default() += 1;
        }
        if let (Some(_), Some(_), None) = (n, w, nw) {
            *corners.entry(*from_node).or_default() += 1;
        }
    }

    Input {
        nodes,
        edges,
        corners,
    }
}

fn get_connected<'a>(input: &'a Input, start_node: &'a Node) -> HashSet<&'a Node> {
    let mut graph = HashSet::<&'a Node>::new();
    let mut heads = HashSet::<&'a Node>::from([start_node]);

    while !heads.is_empty() {
        let next_heads = heads
            .iter()
            .flat_map(|node| input.edges.get(node))
            .flatten()
            .filter(|node| !graph.contains(node))
            .collect();

        graph.extend(heads);
        heads = next_heads;
    }

    graph
}

fn get_plots(input: &Input) -> Vec<HashSet<&Node>> {
    let mut plots: Vec<HashSet<&Node>> = vec![];

    let mut nodes_left: HashSet<&Node> = input.nodes.iter().collect();

    while let Some(node) = nodes_left.iter().next() {
        let area = get_connected(input, node);

        for node in area.iter().cloned() {
            nodes_left.remove(node);
        }
        plots.push(area);
    }
    plots
}

fn get_perimeter(plot: &HashSet<&Node>, input: &Input) -> usize {
    plot.iter()
        .map(|node| input.edges.get(node).cloned().unwrap_or_default())
        .map(|edges| 4 - edges.len())
        .sum()
}

fn get_sides(plot: &HashSet<&Node>, input: &Input) -> usize {
    plot.iter()
        .map(|node| input.corners.get(node).cloned().unwrap_or_default())
        .sum()
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    let plots = get_plots(input);

    plots
        .iter()
        .map(|plot| (plot.len(), get_perimeter(plot, input)))
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
    let plots = get_plots(input);

    plots
        .iter()
        .map(|plot| (plot.len(), get_sides(plot, input)))
        .map(|(area, sides)| area * sides)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC
";
    static TEST_INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    static TEST_INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    static TEST_INPUT_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 140);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 772);
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 80);
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 436);
        assert_eq!(part2(&parse_input(TEST_INPUT_4)), 236);
    }
}
