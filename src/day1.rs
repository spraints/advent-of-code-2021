use super::common;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    for line in common::read_lines(r) {
        println!("line: {}", line);
    }
}
