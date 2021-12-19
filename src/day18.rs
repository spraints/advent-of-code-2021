use super::common::read_lines::read_lines;
use std::io::Read;

type Value = usize;

#[derive(Clone)]
enum SN {
    Leaf(Value),
    Pair(Box<SN>, Box<SN>),
}

pub fn run<R: Read>(r: R) {
    let trees = read_lines(r)
        .map(|line| parse_tree(&line).0)
        .collect::<Vec<SN>>();
    let part1 = trees
        .iter()
        .cloned()
        .reduce(|a, b| reduce(sum(a, b)))
        .unwrap();
    println!("part 1: {}", mag(&part1));
}

fn parse_tree(line: &str) -> (SN, &str) {
    if c0(line) == '[' {
        let (l, line) = parse_tree(&line[1..]);
        assert_eq!(',', c0(line));
        let (r, line) = parse_tree(&line[1..]);
        assert_eq!(']', c0(line));
        (pair(l, r), &line[1..])
    } else {
        let i = line.find(|c| c == ']' || c == ',').unwrap();
        let val = line[0..i].parse().unwrap();
        let line = &line[i..];
        (SN::Leaf(val), line)
    }
}

fn pair(l: SN, r: SN) -> SN {
    SN::Pair(Box::new(l), Box::new(r))
}

fn c0(line: &str) -> char {
    line.chars().next().unwrap()
}

fn sum(l: SN, r: SN) -> SN {
    pair(l, r)
}

fn reduce(mut root: SN) -> SN {
    loop {
        println!("{:?}", &root);
        if let Some(new_root) = finish_exploding(explode(&root, 0)) {
            root = new_root;
            continue;
        }
        if let Some(new_root) = split(&root) {
            root = new_root;
            continue;
        }
        return root;
    }
}

fn split(node: &SN) -> Option<SN> {
    match node {
        SN::Pair(l, r) => match split(l) {
            Some(nl) => Some(pair(nl, unbox(r))),
            None => match split(r) {
                Some(nr) => Some(pair(unbox(l), nr)),
                None => None,
            },
        },
        SN::Leaf(v) if *v > 9 => {
            let l = *v / 2;
            let r = *v - l;
            Some(pair(SN::Leaf(l), SN::Leaf(r)))
        }
        SN::Leaf(_) => None,
    }
}

fn finish_exploding(ex: Explode) -> Option<SN> {
    match ex {
        Explode::Exploded(_, _) => panic!("exploded at root?!?"),
        Explode::Nothing(_) => None,
        Explode::ExplodedLeft(sn, _) => Some(sn),
        Explode::ExplodedRight(sn, _) => Some(sn),
        Explode::ExplodedDone(sn) => Some(sn),
    }
}

enum Explode {
    Nothing(SN),
    Exploded(Value, Value),
    ExplodedLeft(SN, Value),
    ExplodedRight(SN, Value),
    ExplodedDone(SN),
}

fn explode(node: &SN, depth: usize) -> Explode {
    match node {
        SN::Pair(l, r) if l.is_leaf() && r.is_leaf() => {
            if depth > 3 {
                println!("explode!");
                Explode::Exploded(l.value().unwrap(), r.value().unwrap())
            } else {
                Explode::Nothing(pair(unbox(l), unbox(r)))
            }
        }
        SN::Pair(l, r) => {
            match explode(l, depth + 1) {
                Explode::Exploded(el, er) => {
                    // parent needs to add el to its right-most value.
                    Explode::ExplodedLeft(pair(SN::Leaf(0), add_to_left(r, er)), el)
                }
                Explode::ExplodedLeft(nl, el) => {
                    // keep passing el up, parent needs to deal with it.
                    Explode::ExplodedLeft(pair(nl, unbox(r)), el)
                }
                Explode::ExplodedRight(nl, er) => {
                    // we have a value that can accept it!
                    Explode::ExplodedDone(pair(nl, add_to_left(r, er)))
                }
                Explode::ExplodedDone(nl) => Explode::ExplodedDone(pair(nl, unbox(r))),
                Explode::Nothing(nl) => match explode(r, depth + 1) {
                    Explode::Exploded(el, er) => {
                        Explode::ExplodedRight(pair(add_to_right(&nl, el), SN::Leaf(0)), er)
                    }
                    Explode::ExplodedLeft(nr, el) => {
                        Explode::ExplodedDone(pair(add_to_right(&nl, el), nr))
                    }
                    Explode::ExplodedRight(nr, er) => Explode::ExplodedRight(pair(nl, nr), er),
                    Explode::ExplodedDone(nr) => Explode::ExplodedDone(pair(nl, nr)),
                    Explode::Nothing(nr) => Explode::Nothing(pair(nl, nr)),
                },
            }
        }
        n => Explode::Nothing(n.clone()),
    }
}

fn add_to_right(sn: &SN, val: Value) -> SN {
    match sn {
        SN::Pair(l, r) => pair(unbox(l), add_to_right(r, val)),
        SN::Leaf(v) => SN::Leaf(v + val),
    }
}

fn add_to_left(sn: &SN, val: Value) -> SN {
    match sn {
        SN::Pair(l, r) => pair(add_to_left(l, val), unbox(r)),
        SN::Leaf(v) => SN::Leaf(v + val),
    }
}

fn unbox(n: &SN) -> SN {
    n.clone()
}

fn mag(n: &SN) -> Value {
    match n {
        SN::Leaf(x) => *x,
        SN::Pair(l, r) => 3 * mag(l) + 2 * mag(r),
    }
}

impl SN {
    pub fn is_leaf(&self) -> bool {
        match self {
            SN::Leaf(_) => true,
            _ => false,
        }
    }

    pub fn value(&self) -> Option<Value> {
        match self {
            SN::Leaf(v) => Some(*v),
            _ => None,
        }
    }
}

impl std::fmt::Debug for SN {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SN::Pair(l, r) => write!(f, "[{:?},{:?}]", l, r),
            SN::Leaf(v) => write!(f, "{}", v),
        }
    }
}
