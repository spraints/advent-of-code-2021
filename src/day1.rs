use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    for line in read_lines(r) {
        println!("line: {}", line);
    }
}
