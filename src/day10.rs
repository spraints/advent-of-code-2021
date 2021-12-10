use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut part1 = 0;
    let mut part2 = vec![];
    for line in read_lines(r) {
        let (corrupt, incomplete) = analyze(&line);
        part1 += corrupt;
        if let Some(incomplete) = incomplete {
            part2.push(incomplete);
        }
    }
    println!("part 1: {}", part1);
    part2.sort_unstable();
    println!("part 2: {}", part2[part2.len() / 2]);
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
