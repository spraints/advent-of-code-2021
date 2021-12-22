use super::common::read_lines::read_lines;
use std::collections::HashMap;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let map = read_lines(r).map(|line| parse_line(&line)).collect();
    let mut basins = find_basins(map);
    println!(
        "part 1: {}",
        basins
            .iter()
            .map(|basin| (basin.min + 1) as u32)
            .sum::<u32>()
    );
    basins.sort_unstable_by(|a, b| b.size.cmp(&a.size));
    println!(
        "part 2: {}",
        basins
            .iter()
            .take(3)
            .map(|basin| basin.size)
            .product::<usize>()
    );
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[derive(Debug)]
struct Basin {
    size: usize,
    min: u8,
}

fn find_basins(map: Vec<Vec<u8>>) -> Vec<Basin> {
    let mut hmap = HashMap::new();
    for (x, row) in map.into_iter().enumerate() {
        for (y, val) in row.into_iter().enumerate() {
            if val < 9 {
                hmap.insert((x, y), val);
            }
        }
    }
    let mut res = vec![];
    while !hmap.is_empty() {
        let k = *hmap.keys().next().unwrap();
        let mut basin = Basin { min: 9, size: 0 };
        consume_basin(&mut hmap, k, &mut basin);
        assert_ne!(basin.size, 0);
        res.push(basin);
    }
    res
}

fn consume_basin(hmap: &mut HashMap<(usize, usize), u8>, k: (usize, usize), basin: &mut Basin) {
    match hmap.remove(&k) {
        None => (),
        Some(v) => {
            basin.size += 1;
            if v < basin.min {
                basin.min = v;
            }
            let (x, y) = k;
            consume_basin(hmap, (x + 1, y), basin);
            consume_basin(hmap, (x, y + 1), basin);
            if x > 0 {
                consume_basin(hmap, (x - 1, y), basin);
            }
            if y > 0 {
                consume_basin(hmap, (x, y - 1), basin);
            }
        }
    };
}
