use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let (part1, part2) = process(read_lines(r));
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn process<T: Iterator<Item = String>>(lines: T) -> (u64, u64) {
    let (mut depth, mut pos, mut aim) = (0, 0, 0);
    for line in lines {
        let (command, val) = parse_line(&line);
        match command {
            "forward" => {
                pos += val;
                depth += val * aim
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => panic!("unknown: {}", line),
        };
    }
    (aim * pos, depth * pos)
}

fn parse_line(line: &str) -> (&str, u64) {
    let mut parts = line.split(' ');
    let command = parts.next().unwrap();
    let val = parts.next().unwrap().parse().unwrap();
    (command, val)
}
