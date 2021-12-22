use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut octopi = [[0; 10]; 10];
    for (i, line) in read_lines(r).enumerate() {
        octopi[i] = parse_line(&line).unwrap();
    }

    let mut steps = 0;
    let mut flashes = 0;
    loop {
        let (next, flashed) = step(octopi);
        steps += 1;
        flashes += flashed;
        octopi = next;
        if steps == 100 {
            println!("part 1: {}", flashes);
        }
        if flashed == 100 {
            println!("part 2: {}", steps);
            return;
        }
    }
}

fn parse_line(line: &str) -> Result<[u8; 10], String> {
    let mut row = [0; 10];
    for (j, c) in line.chars().enumerate() {
        if j > 9 {
            return Err(format!("too many chars ({}) in {}", c, line));
        }
        row[j] = (c as u8) - b'0';
    }
    Ok(row)
}

fn step(mut octopi: [[u8; 10]; 10]) -> ([[u8; 10]; 10], usize) {
    let mut flashed = vec![];
    for (i, row) in octopi.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            *val += 1;
            if *val == 10 {
                flashed.push((i, j));
            }
        }
    }
    let mut x = 0;
    while x < flashed.len() {
        for (i, j) in neighbors(flashed[x]) {
            octopi[i][j] += 1;
            if octopi[i][j] == 10 {
                flashed.push((i, j));
            }
        }
        x += 1;
    }
    for (i, j) in flashed.iter() {
        octopi[*i][*j] = 0;
    }
    (octopi, flashed.len())
}

fn neighbors(coords: (usize, usize)) -> Neighbors {
    Neighbors {
        coords: (coords.0 as isize, coords.1 as isize),
        di: -1,
        dj: -1,
    }
}

struct Neighbors {
    coords: (isize, isize),
    di: isize,
    dj: isize,
}

impl Iterator for Neighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.di > 1 || self.dj > 1 {
            None
        } else {
            let (ci, cj) = self.coords;
            let (i, j) = ((ci + self.di), (cj + self.dj));
            if self.dj == 1 {
                self.di += 1;
                self.dj = -1;
            } else {
                self.dj += 1;
            }
            if (i == ci && j == cj) || i < 0 || i > 9 || j < 0 || j > 9 {
                self.next()
            } else {
                Some((i as usize, j as usize))
            }
        }
    }
}
