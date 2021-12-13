use super::common::read_lines::read_lines;
use std::collections::{HashMap, HashSet};
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut edges = HashMap::new();
    for line in read_lines(r) {
        let (from, to) = line.split_once('-').unwrap();
        add(&mut edges, &from, &to);
        add(&mut edges, &to, &from);
    }

    println!("part 1: {}", count_paths(&edges, false));
    println!("part 2: {}", count_paths(&edges, true));
}

fn count_paths(edges: &HashMap<String, Vec<String>>, double: bool) -> usize {
    let p = "start";
    let mut visited = HashSet::new();
    let mut path = vec![];
    count_paths2(edges, &p, double, &mut visited, &mut path)
}

fn count_paths2(
    edges: &HashMap<String, Vec<String>>,
    p: &str,
    double: bool,
    visited: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> usize {
    if p.chars().all(|c| c.is_lowercase()) {
        visited.insert(p.to_string());
    }
    path.push(p.to_string());
    //println!("path: {:?}; visited: {:?}", path, visited);

    let mut count = 0;
    for n in edges[p].iter() {
        //println!("  {:?} ok? {}, {}", path, n, double);
        if n.eq("end") {
            //println!("  ^ end!");
            count += 1;
        } else if n.eq("start") {
        } else if visited.contains(n) {
            if double {
                //println!("  ^ double visit small cave!");
                count += count_paths2(edges, n, false, visited, path);
                visited.insert(n.to_string());
            }
        } else {
            //println!("  ^ ok!");
            count += count_paths2(edges, n, double, visited, path);
        }
    }

    visited.remove(p);
    path.pop();

    count
}

fn add<'a>(edges: &mut HashMap<String, Vec<String>>, from: &str, to: &str) {
    let e = edges.entry(from.to_string()).or_insert_with(|| vec![]);
    e.push(to.to_string());
}
