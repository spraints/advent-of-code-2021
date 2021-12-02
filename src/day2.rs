use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let lines: Vec<String> = read_lines(r).collect();
    let (part1, part2) = process(&lines);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn process(lines: &Vec<String>) -> (u64, u64) {
    let (mut depth, mut pos, mut aim) = (0, 0, 0);
    for line in lines {
        let mut parts = line.split(' ');
        let command = parts.next().unwrap();
        let val: u64 = parts.next().unwrap().parse().unwrap();
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
