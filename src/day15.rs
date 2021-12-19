use super::common::read_lines::read_lines;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let lines = read_lines(r)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let (graph, dest) = make_graph(&lines);
    println!("part 1: {}", least_cost(&graph, dest));
}

fn least_cost(graph: &Graph, dest: Node) -> u32 {
    let mut unvisited: HashSet<Node> = graph.keys().cloned().collect();

    let mut tentative_distance = HashMap::new();
    let mut cur = Node(0, 0);
    tentative_distance.insert(cur, 0);

    loop {
        // println!(
        //     "visiting: {:?} ({} unvisited nodes out of {})",
        //     cur,
        //     unvisited.len(),
        //     graph.len()
        // );

        let dist = *tentative_distance.get(&cur).unwrap();
        for e in graph.get(&cur).unwrap() {
            if unvisited.contains(&e.to) {
                let this_dist = dist + e.cost;
                let edist = tentative_distance.entry(e.to).or_insert(999_999_999);
                //println!("  here to {:?} was {} could be {}", e.to, *edist, this_dist);
                if this_dist < *edist {
                    //println!("   updated!");
                    *edist = this_dist;
                }
            }
        }

        unvisited.remove(&cur);

        if !unvisited.contains(&dest) {
            //println!("dest ({:?}) has been visited!!", dest);
            return *tentative_distance.get(&dest).unwrap();
        }

        cur = *unvisited
            .iter()
            .reduce(|a, b| match tentative_distance.get(a) {
                None => b,
                Some(costa) => match tentative_distance.get(b) {
                    None => a,
                    Some(costb) => {
                        if costa < costb {
                            a
                        } else {
                            b
                        }
                    }
                },
            })
            .unwrap();
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Node(usize, usize);

type Graph = HashMap<Node, Vec<Edge>>;

struct Edge {
    to: Node,
    cost: u32,
}

type Lines = Vec<Vec<u32>>;

fn make_graph(lines: &Lines) -> (Graph, Node) {
    let mut res = HashMap::new();
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            let node = Node(i, j);
            let mut edges = vec![];
            add_edge(&mut edges, Node(i + 1, j), lines);
            add_edge(&mut edges, Node(i, j + 1), lines);
            res.insert(node, edges);
        }
    }
    (res, Node(lines.len() - 1, lines[0].len() - 1))
}

fn add_edge(edges: &mut Vec<Edge>, to: Node, lines: &Lines) {
    if let Some(line) = lines.get(to.0) {
        if let Some(cost) = line.get(to.1) {
            edges.push(Edge { to, cost: *cost });
        }
    }
}
