use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut part1 = 0;
    let mut part2 = vec![];
    for line in read_lines(r) {
        let (corrupt, incomplete) = analyze(&line);
        part1 += corrupt;
        if let Some(incomplete) = incomplete {
            println!("{} => {}", line, incomplete);
            part2.push(incomplete);
        }
    }
    println!("part 1: {}", part1);
    part2.sort_unstable();
    println!("part 2: {}", part2[part2.len() / 2]);
}

pub fn run_int<R: Read>(r: R) {
    let mut part1 = 0;
    let mut part2 = vec![];
    for line in read_lines(r) {
        match analyze_int(&line) {
            Score::Corrupt(score) => part1 += score,
            Score::Incomplete(score) => {
                println!("{} => {}", line, score);
                part2.push(score);
            }
        };
    }
    println!("part 1: {}", part1);
    part2.sort_unstable();
    println!("part 2: {}", part2[part2.len() / 2]);
}

enum Score {
    Corrupt(u64),
    Incomplete(u64),
}

enum Char {
    Open(u64),
    Close(u64, u64),
}

fn analyze_int(line: &str) -> Score {
    let mut incomplete = 0;
    for c in line.chars() {
        match analyze_char(c) {
            Char::Open(ac_score) => incomplete = incomplete * 5 + ac_score,
            Char::Close(ac_score, c_score) => {
                if incomplete % 5 == ac_score {
                    incomplete /= 5;
                } else {
                    return Score::Corrupt(c_score);
                }
            }
        };
    }
    Score::Incomplete(incomplete)
}

fn analyze_char(c: char) -> Char {
    match c {
        '(' => Char::Open(1),
        ')' => Char::Close(1, 3),
        '[' => Char::Open(2),
        ']' => Char::Close(2, 57),
        '{' => Char::Open(3),
        '}' => Char::Close(3, 1197),
        '<' => Char::Open(4),
        '>' => Char::Close(4, 25137),
        _ => panic!("wut {}", c),
    }
}

fn analyze(line: &str) -> (u64, Option<u64>) {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => match (stack.pop().unwrap(), c) {
                ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => (),
                (_, ')') => return (3, None),
                (_, ']') => return (57, None),
                (_, '}') => return (1197, None),
                (_, '>') => return (25137, None),
                _ => panic!(":("),
            },
        };
    }
    let mut score = 0;
    loop {
        score = score * 5
            + match stack.pop() {
                None => return (0, Some(score)),
                Some('(') => 1,
                Some('[') => 2,
                Some('{') => 3,
                Some('<') => 4,
                _ => panic!(":("),
            };
    }
}
