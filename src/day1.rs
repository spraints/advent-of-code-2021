use super::common::parse_lines::parse_lines;
use std::io::Read;

pub fn run<R: Read>(mut r: R) {
    let nums: Vec<u16> = parse_lines(r);
    let mut last = None;
    let mut count = 0;
    for num in nums {
        if let Some(n) = last {
            if num > n {
                count = count + 1;
            }
        }
        last = Some(num);
    }
    println!("part1: {}", count);
}
