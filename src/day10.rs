use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut part1 = 0;
    let mut part2 = vec![];
    for line in read_lines(r) {
        let (corrupt, incomplete) = analyze(&line);
        part1 += corrupt;
        if let Some(incomplete) = incomplete {
            //println!("{} => {}", line, incomplete);
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
                part2.push(score);
            }
        };
    }
    println!("part 1: {}", part1);
    part2.sort_unstable();
    println!("part 2: {}", part2[part2.len() / 2]);
}

pub fn run_rec<R: Read>(r: R) {
    let mut part1 = 0;
    let mut part2 = vec![];
    for line in read_lines(r) {
        //println!("{}", line);
        match analyze_rec(line.chars()).unwrap() {
            Score::Corrupt(score) => part1 += score,
            Score::Incomplete(score) => {
                //println!("incomplete: {}", score);
                part2.push(score);
            }
        };
    }
    println!("part 1: {}", part1);
    part2.sort_unstable();
    println!("part 2: {}", part2[part2.len() / 2]);
}

fn analyze_rec<I: Iterator<Item = char>>(mut line: I) -> Option<Score> {
    match want_close(&mut line, 0, 0) {
        Some(Score::Incomplete(x)) => Some(Score::Incomplete(x / 5)),
        x => x,
    }
}

fn want_close<I: Iterator<Item = char>>(line: &mut I, open: u64, depth: usize) -> Option<Score> {
    //println!("[{}] open: {}", depth, open);
    loop {
        match line.next() {
            None => {
                //println!("[{}] didn't find close for {}", depth, open);
                return Some(Score::Incomplete(open));
            }
            Some(c) => match analyze_char(c) {
                Char::Close(ac_score, c_score) => {
                    if ac_score == open {
                        //println!("[{}] close: {} {}", depth, c, ac_score);
                        return None;
                    } else {
                        //println!("[{}] unexpected: {} {}", depth, c, ac_score);
                        return Some(Score::Corrupt(c_score));
                    }
                }
                Char::Open(x) => match want_close(line, x, depth + 1) {
                    Some(Score::Incomplete(score)) => {
                        //println!("[{}] also didn't find close for {}", depth, open);
                        return Some(Score::Incomplete(score * 5 + open));
                    }
                    Some(score) => return Some(score),
                    None => (),
                },
            },
        };
    }
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
    let mut scale = 1;
    for c in line.chars() {
        match analyze_char(c) {
            Char::Open(ac_score) => {
                incomplete += scale * ac_score;
                scale *= 5;
            }
            Char::Close(ac_score, c_score) => {
                scale /= 5;
                if incomplete / scale == ac_score {
                    incomplete %= scale;
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
