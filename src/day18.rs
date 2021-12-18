use super::common::read_lines::read_lines;
use std::io::Read;

#[derive(Clone)]
enum Node {
    Leaf(usize),
    Tree(Box<Node>, Box<Node>),
}

pub fn run<R: Read>(r: R) {
    let trees = read_lines(r)
        .map(|line| parse_tree(&line).0)
        .collect::<Vec<Node>>();
    let part1 = trees
        .iter()
        .cloned()
        .reduce(|a, b| reduce(sum(a, b)))
        .unwrap();
    println!("part 1: {}", mag(&part1));
}

fn parse_tree(line: &str) -> (Node, &str) {
    if c0(line) == '[' {
        println!("new node at {}", line);
        let (l, line) = parse_tree(&line[1..]);
        assert_eq!(',', c0(line));
        let (r, line) = parse_tree(&line[1..]);
        assert_eq!(']', c0(line));
        (Node::Tree(Box::new(l), Box::new(r)), &line[1..])
    } else {
        println!("new value at {}", line);
        let i = line.find(|c| c == ']' || c == ',').unwrap();
        let val = line[0..i].parse().unwrap();
        let line = &line[i..];
        println!(" --> {} // {}", val, line);
        (Node::Leaf(val), line)
    }
}

fn c0(line: &str) -> char {
    line.chars().next().unwrap()
}

fn sum(l: Node, r: Node) -> Node {
    Node::Tree(Box::new(l), Box::new(r))
}

fn reduce(node: Node) -> Node {
    // TODO
    node
}

fn mag(n: &Node) -> usize {
    match n {
        Node::Leaf(x) => *x,
        Node::Tree(l, r) => 3 * mag(l) + 2 * mag(r),
    }
}
