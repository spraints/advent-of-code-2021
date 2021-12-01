use super::common::parse_lines::parse_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let nums: Vec<u16> = parse_lines(r);
    println!("part 1: {}", count(&nums, 2));
    println!("part 2: {}", count(&nums, 4));
}

fn count(v: &Vec<u16>, sz: usize) -> usize {
    v.windows(sz)
        .filter(|nums| nums.last() > nums.first())
        .count()
}
