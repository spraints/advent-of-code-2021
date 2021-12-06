use super::common::read_lines::read_lines;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut by_age: [usize; 9] = [0; 9];
    let line = read_lines(r).next().unwrap();
    for age in line.split(',') {
        let age: usize = age.parse().unwrap();
        by_age[age] += 1;
    }
    for _ in 0..80 {
        let babies = by_age[0];
        for i in 1..9 {
            by_age[i - 1] = by_age[i];
        }
        by_age[6] += babies;
        by_age[8] = babies;
    }
    println!(
        "part 1: {}",
        by_age.iter().map(|a| *a).reduce(|a, b| a + b).unwrap()
    );
    for _ in 80..256 {
        let babies = by_age[0];
        for i in 1..9 {
            by_age[i - 1] = by_age[i];
        }
        by_age[6] += babies;
        by_age[8] = babies;
    }
    println!(
        "part 2: {}",
        by_age.iter().map(|a| *a).reduce(|a, b| a + b).unwrap()
    );
}
