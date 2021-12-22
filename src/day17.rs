use super::common::read_lines::read_lines;
use std::io::Read;
use std::ops::RangeInclusive;

pub fn run<R: Read>(r: R) {
    let line = read_lines(r).next().unwrap();
    let mut words = line.split(' ').skip(2);
    let (min_x, max_x) = parse_range(words.next().unwrap());
    let (min_y, max_y) = parse_range(words.next().unwrap());
    let xr = min_x..=max_x;
    let yr = min_y..=max_y;

    let mut hits = 0;
    let mut max_dy = 0;
    for dx in 0..(max_x + 1) {
        for dy in min_y..1 - min_y {
            if is_hit((dx, dy), (xr.clone(), yr.clone())) {
                hits += 1;
                if dy > max_dy {
                    max_dy = dy
                }
            }
        }
    }

    println!("part 1: {}", max_dy * (max_dy + 1) / 2);
    println!("part 2: {}", hits);
}

fn is_hit(dir: (isize, isize), range: (RangeInclusive<isize>, RangeInclusive<isize>)) -> bool {
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = dir;
    let (xr, yr) = range;
    loop {
        if xr.contains(&x) && yr.contains(&y) {
            return true;
        }
        if x > *xr.end() || y < *yr.start() {
            return false;
        }
        x += dx;
        y += dy;
        if dx > 0 {
            dx -= 1;
        }
        dy -= 1;
    }
}

fn parse_range(s: &str) -> (isize, isize) {
    let s = s.trim_end_matches(',');
    let (_, s) = s.split_once('=').unwrap();
    let (min, max) = s.split_once("..").unwrap();
    (min.parse().unwrap(), max.parse().unwrap())
}
