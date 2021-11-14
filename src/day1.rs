use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(mut r: R) {
    let mut s = String::new();
    r.read_to_string(&mut s).unwrap();
    println!("end on floor: {}", part1(&s));
}

fn part1(s: &str) -> i32 {
    s.chars().fold(0, |floor, c| floor + dir(c))
}

fn dir(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

#[test]
fn test_part1() {
    assert_eq!(0, part1("(())"));
    assert_eq!(0, part1("()()"));
    assert_eq!(3, part1("((("));
    assert_eq!(3, part1("(()(()("));
}
