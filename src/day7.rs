use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut positions: Vec<i64> = read_lines(r)
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    positions.sort();
    println!(
        "part 1: {}",
        cost1(&positions, positions[positions.len() / 2])
    );

    println!("part 2: {}", cost2(&positions, mean(&positions)));
}

fn cost1(positions: &[i64], dest: i64) -> i64 {
    positions.iter().map(|pos| (*pos - dest).abs()).sum()
}

fn cost2(positions: &[i64], dest: i64) -> i64 {
    positions
        .iter()
        .map(|pos| (*pos - dest).abs())
        .map(|dist| dist * (dist + 1) / 2)
        .sum()
}

fn mean(positions: &[i64]) -> i64 {
    positions.iter().sum::<i64>() / positions.len() as i64
}
