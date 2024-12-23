use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

type Nodes<'a> = BTreeSet<&'a String>;
type Graph = HashMap<String, BTreeSet<String>>;
#[derive(Debug)]
struct Input {
    graph: Graph,
}

/// Get all complete sub-graphs using the Bron-Kerbosch algorithm
fn get_complete_sub_graphs<'a>(
    graph: &'a Graph,
    r: Nodes<'a>,
    mut p: Nodes<'a>,
    mut x: Nodes<'a>,
    cliques: &mut Vec<Nodes<'a>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    }

    for node in p.iter().cloned().collect_vec() {
        let neighbours: Nodes = graph.get(node).unwrap().iter().collect();
        let mut new_r = r.clone();
        new_r.insert(node);
        get_complete_sub_graphs(
            graph,
            new_r,
            p.intersection(&neighbours).cloned().collect(),
            x.intersection(&neighbours).cloned().collect(),
            cliques,
        );
        p.remove(node);
        x.insert(node);
    }
}

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Input {
    let mut graph: HashMap<String, BTreeSet<String>> = HashMap::new();

    for line in input.lines() {
        let a = line[0..2].to_string();
        let b = line[3..].to_string();

        // Non-directional graph
        graph.entry(a.clone()).or_default().insert(b.clone());
        graph.entry(b).or_default().insert(a);
    }

    Input { graph }
}

#[aoc(day23, part1)]
fn part1(input: &Input) -> usize {
    let t_nodes = input
        .graph
        .keys()
        .filter(|n| n.starts_with('t'))
        .collect::<Vec<_>>();

    let mut triples = HashSet::new();

    for a in t_nodes {
        let bs = input.graph.get(a).unwrap();
        for b in bs {
            let cs = input.graph.get(b).unwrap();

            for c in cs {
                if input.graph.get(a).unwrap().contains(c) {
                    let mut triple = [a, b, c];
                    triple.sort();

                    triples.insert(triple);
                }
            }
        }
    }

    triples.len()
}

#[aoc(day23, part2)]
fn part2(input: &Input) -> String {
    let mut cliques = Vec::new();

    get_complete_sub_graphs(
        &input.graph,
        Nodes::new(),
        input.graph.keys().collect(),
        Nodes::new(),
        &mut cliques,
    );

    let largest_network = cliques
        .iter()
        .max_by(|c1, c2| c1.len().cmp(&c2.len()))
        .unwrap();

    largest_network.iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), "co,de,ka,ta");
    }
}
