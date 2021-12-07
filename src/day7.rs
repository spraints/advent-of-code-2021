use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut positions: Vec<i64> = read_lines(r)
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    positions.sort_unstable();
    println!(
        "part 1: {}",
        cost1(&positions, positions[positions.len() / 2])
    );
    let min = *positions.first().unwrap();
    let max = *positions.last().unwrap();
    let mut mincost = None;
    for pos in min..=max {
        let cost = cost2(&positions, pos);
        mincost = Some(match mincost {
            None => cost,
            Some(x) if x > cost => cost,
            Some(x) => x,
        });
    }
    println!("part 2: {}", mincost.unwrap());
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
