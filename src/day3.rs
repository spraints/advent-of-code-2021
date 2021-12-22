use super::common::read_lines::read_lines;
use std::io::Read;

type ParsedLine = (Vec<char>, u32);

pub fn run<R: Read>(r: R) {
    let numbers: Vec<ParsedLine> = read_lines(r)
        .map(|s| (s.chars().collect(), u32::from_str_radix(&s, 2).unwrap()))
        .collect();

    let (gamma, epsilon) = rates(&numbers);
    println!("part 1: {}", gamma * epsilon);

    let o2 = search(&numbers, true);
    let co2 = search(&numbers, false);
    println!("part 2: {}", o2 * co2);
}

fn rates(numbers: &[ParsedLine]) -> (u32, u32) {
    let mut ones = vec![];
    ones.resize(numbers[0].0.len(), 0);
    for (bits, _) in numbers {
        for (i, bit) in bits.iter().enumerate() {
            if *bit == '1' {
                ones[i] += 1;
            }
        }
    }
    ones.into_iter().fold((0, 0), |(g, e), count| {
        if count > numbers.len() / 2 {
            (g * 2 + 1, e * 2)
        } else {
            (g * 2, e * 2 + 1)
        }
    })
}

type Vals = Vec<(usize, u32)>;

fn search(n: &[ParsedLine], want_most: bool) -> u32 {
    let mut vals: Vals = n.iter().map(|(_, val)| *val).enumerate().collect();
    let mut i = 0;
    loop {
        let (zeros, ones): (Vals, Vals) = vals.into_iter().partition(|(ni, _)| n[*ni].0[i] == '0');
        #[allow(clippy::collapsible_else_if)]
        if zeros.len() > ones.len() {
            if want_most {
                vals = zeros;
            } else {
                vals = ones;
            }
        } else {
            if want_most {
                vals = ones;
            } else {
                vals = zeros;
            }
        }
        if vals.len() == 1 {
            return vals[0].1;
        }
        i += 1;
    }
}
