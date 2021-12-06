use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut by_age: [usize; 9] = [0; 9];
    let line = read_lines(r).next().unwrap();
    for age in line.split(',') {
        let age: usize = age.parse().unwrap();
        by_age[age] += 1;
    }
    let (by_age, count) = run_days(by_age, 80);
    println!("part 1: {}", count);
    let (by_age, count) = run_days(by_age, 256 - 80);
    println!("part 2: {}", count);
}

fn run_days(mut by_age: [usize; 9], n: usize) -> ([usize; 9], usize) {
    for _ in 0..n {
        let babies = by_age[0];
        for i in 1..9 {
            by_age[i - 1] = by_age[i];
        }
        by_age[6] += babies;
        by_age[8] = babies;
    }
    let count = by_age.iter().map(|a| *a).reduce(|a, b| a + b).unwrap();
    (by_age, count)
}
