use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let numbers: Vec<(Vec<char>, u32)> = read_lines(r)
        .map(|s| (s.chars().collect(), u32::from_str_radix(&s, 2).unwrap()))
        .collect();

    let (gamma, epsilon) = rates(&numbers);
    println!("part 1: {}", gamma * epsilon);
}

fn rates(numbers: &Vec<(Vec<char>, u32)>) -> (u32, u32) {
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
