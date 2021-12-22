use super::common::read_sections::read_sections;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut sections = read_sections(r);
    let (mut pair_counts, mut elem_counts) =
        parse_init(sections.next().unwrap().into_iter().next().unwrap());
    let rules = sections
        .next()
        .unwrap()
        .into_iter()
        .map(parse_rule)
        .map(|rule| (rule.parent, rule))
        .collect::<HashMap<Pair, Rule>>();

    for _ in 0..10 {
        let (p, e) = step(pair_counts, elem_counts, &rules);
        pair_counts = p;
        elem_counts = e;
    }
    println!("part 1: {}", score(&elem_counts));

    for _ in 0..30 {
        let (p, e) = step(pair_counts, elem_counts, &rules);
        pair_counts = p;
        elem_counts = e;
    }
    println!("part 2: {}", score(&elem_counts));
}

fn step(
    pair_counts: PairCounts,
    mut elem_counts: ElemCounts,
    rules: &HashMap<Pair, Rule>,
) -> (PairCounts, ElemCounts) {
    let mut new_pair_counts = PairCounts::new();
    for (pair, count) in pair_counts {
        match rules.get(&pair) {
            None => {
                add(&mut new_pair_counts, pair, count);
            }
            Some(rule) => {
                add(&mut new_pair_counts, rule.baby1, count);
                add(&mut new_pair_counts, rule.baby2, count);
                add(&mut elem_counts, rule.ins, count);
            }
        };
    }
    (new_pair_counts, elem_counts)
}

fn score(counts: &ElemCounts) -> usize {
    let (mut min, mut max) = (usize::MAX, 0);
    for count in counts.values() {
        let count = *count;
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }
    max - min
}

type Pair = u16;
type Elem = char;

type PairCounts = HashMap<Pair, usize>;
type ElemCounts = HashMap<Elem, usize>;

struct Rule {
    parent: Pair,
    baby1: Pair,
    baby2: Pair,
    ins: Elem,
}

fn parse_init(line: String) -> (PairCounts, ElemCounts) {
    let mut pc = PairCounts::new();
    let mut ec = ElemCounts::new();
    let mut last = None;
    for c in line.chars() {
        add(&mut ec, c, 1);
        if let Some(l) = last {
            add(&mut pc, make_pair(l, c), 1);
        }
        last = Some(c);
    }
    (pc, ec)
}

fn parse_rule(line: String) -> Rule {
    let (pair, ins) = line.split_once(" -> ").unwrap();
    let mut pair = pair.chars();
    let a = pair.next().unwrap();
    let b = pair.next().unwrap();
    let ins = ins.chars().next().unwrap();
    Rule {
        parent: make_pair(a, b),
        baby1: make_pair(a, ins),
        baby2: make_pair(ins, b),
        ins,
    }
}

fn make_pair(a: char, b: char) -> Pair {
    (a as u16) << 8 | (b as u16)
}

fn add<K: Eq + Hash>(hm: &mut HashMap<K, usize>, k: K, delta: usize) {
    let e = hm.entry(k).or_insert(0);
    *e += delta;
}
