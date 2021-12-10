use super::common::read_lines::read_lines;
use std::collections::HashMap;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let lines: Vec<Line> = read_lines(r)
        .map(|line| parse_line(&line))
        .collect::<Result<Vec<Line>, _>>()
        .unwrap();

    println!("part 1: {}", count_simple(&lines));
    let nums = lines
        .iter()
        .map(|line| decode(line))
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();
    println!("part 2: {}", nums.into_iter().sum::<u32>());
}

pub fn run_ttaylor<R: Read>(r: R) {
    let lines: Vec<Line> = read_lines(r)
        .map(|line| parse_line(&line))
        .collect::<Result<Vec<Line>, _>>()
        .unwrap();

    println!("part 1: {}", count_simple(&lines));
    let part2 = lines.iter().map(|line| decode2(line)).sum::<usize>();
    println!("part 2: {}", part2);
}

// This is based on ttaylor's solution (ruby):
//
// $seed = [1, 4, 7, 8]
// $segments = [
//   "abcefg",
//   "cf",
//   "acdeg",
//   "acdfg",
//   "bcdf",
//   "abdfg",
//   "abdefg",
//   "acf",
//   "abcdefg",
//   "abcdfg",
// ].map(&:chars)
//
// def find_signal(decoded, signals, n)
//   matching = signals.filter { |s| s.length == $segments[n].length }
//   matching.detect do |signal|
//     decoded.each_with_index \
//       .reject { |d, _| d.nil? } \
//       .all? { |d, i| (signal & d).length == ($segments[n] & $segments[i]).length }
//   end
// end
//
// part_1 = 0
// part_2 = 0
//
// File.readlines("input.txt").each do |x|
//   decoded = Array.new(10)
//
//   signals, out = x.split(" | ").map { |str| str.split(" ").map { |s| s.chars.to_set } }
//
//   order = $seed + (10.times.to_a - $seed)
//   order.each { |i| decoded[i] = find_signal(decoded, signals, i) }
//
//   part_1 += out.count { |x| $seed.include? decoded.index(x) }
//   part_2 += out.inject(0) { |got, x| got * 10 + decoded.index(x) }
// end
fn decode2(line: &Line) -> usize {
    let mut decoded = [0; 10];
    for i in ORDER {
        decoded[i] = find_signal(&decoded, &line.key, i).unwrap();
    }
    //println!("{:?}", decoded);
    line.code.iter().fold(0, |code, digit| {
        code * 10 + decoded.iter().position(|x| x == digit).unwrap()
    })
}

const ORDER: [usize; 10] = [1, 4, 7, 8, 0, 2, 3, 5, 6, 9];
const REFERENCE: [u8; 10] = [
    0b1110111, //   "abcefg",
    0b0010010, //   "cf",
    0b1011101, //   "acdeg",
    0b1011011, //   "acdfg",
    0b0111010, //   "bcdf",
    0b1101011, //   "abdfg",
    0b1101111, //   "abdefg",
    0b1010010, //   "acf",
    0b1111111, //   "abcdefg",
    0b1111011, //   "abcdfg",
];

fn find_signal(decoded: &[u8], key: &Key, n: usize) -> Option<u8> {
    key.iter()
        .find(|x| {
            x.count_ones() == REFERENCE[n].count_ones()
                && decoded
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| **d != 0)
                    .all(|(i, d)| {
                        (*x & d).count_ones() == (REFERENCE[n] & REFERENCE[i]).count_ones()
                    })
        })
        .and_then(|x| Some(*x))
}

struct Line {
    key: Key,
    code: Code,
}
type Key = [u8; 10];
type Code = [u8; 4];

fn parse_line(line: &str) -> Result<Line, String> {
    let (keybits, codebits) = line.split_once('|').ok_or("missing separator")?;
    let (mut key, mut code) = ([0_u8; 10], [0_u8; 4]);
    parse_bits(&mut key, keybits)?;
    parse_bits(&mut code, codebits)?;
    Ok(Line { key, code })
}

fn parse_bits(out: &mut [u8], line: &str) -> Result<(), String> {
    for (i, bits) in line.trim().split(' ').enumerate() {
        if i >= out.len() {
            return Err(format!("expected {} words in {}", out.len(), line));
        }
        out[i] = bits
            .chars()
            .map(|c| match c {
                'a' => Ok(0b0000001),
                'b' => Ok(0b0000010),
                'c' => Ok(0b0000100),
                'd' => Ok(0b0001000),
                'e' => Ok(0b0010000),
                'f' => Ok(0b0100000),
                'g' => Ok(0b1000000),
                _ => Err(format!("can't parse {}", bits)),
            })
            .try_fold(0, |a, b| b.and_then(|b| Ok(a | b)))?;
    }
    Ok(())
}

fn count_simple(lines: &[Line]) -> u32 {
    lines
        .iter()
        .map(|line| {
            line.code
                .iter()
                .filter(|code| match count_bits(code) {
                    5 | 6 => false,
                    _ => true,
                })
                .count() as u32
        })
        .sum()
}

fn count_bits(code: &u8) -> u32 {
    code.count_ones()
}

fn decode(line: &Line) -> Result<u32, String> {
    let mut rest = line.key.clone();
    let one = extract(&mut rest, |n| n.count_ones() == 2)?;
    let seven = extract(&mut rest, |n| n.count_ones() == 3)?;
    let four = extract(&mut rest, |n| n.count_ones() == 4)?;
    let eight = extract(&mut rest, |n| n.count_ones() == 7)?;
    // 9 - overlaps with 4.
    let nine = extract(&mut rest, |n| n & four == four)?;
    // 0 - overlaps with 1 and has 6 bits
    let zero = extract(&mut rest, |n| n & one == one && n.count_ones() == 6)?;
    // 6 - 6 bits
    let six = extract(&mut rest, |n| n.count_ones() == 6)?;
    // 3 - overlaps with 1
    let three = extract(&mut rest, |n| n & one == one)?;
    // 5 - overlaps with 9
    let five = extract(&mut rest, |n| n > 0 && n & nine == n)?;
    // 2 - 5 bits
    let two = extract(&mut rest, |n| n > 0)?;
    //println!("rest: {:?}", rest);
    let mut key = HashMap::new();
    key.insert(zero, 0_u32);
    key.insert(one, 1);
    key.insert(two, 2);
    key.insert(three, 3);
    key.insert(four, 4);
    key.insert(five, 5);
    key.insert(six, 6);
    key.insert(seven, 7);
    key.insert(eight, 8);
    key.insert(nine, 9);
    //println!("key: {:?}", key);
    //println!("code: {:?}", line.code);
    Ok(line
        .code
        .iter()
        .map(|n| key[n])
        .reduce(|a, b| a * 10 + b)
        .unwrap())
}

fn extract<F>(xs: &mut [u8], f: F) -> Result<u8, String>
where
    F: Fn(u8) -> bool,
{
    for x in xs.iter_mut() {
        if f(*x) {
            let ret = *x;
            *x = 0;
            return Ok(ret);
        }
    }
    Err(format!(
        "no match for f() in {:?}",
        xs.iter().filter(|n| **n > 0)
    ))
}

#[test]
fn test_decode() {
    let line = Line {
        // these are backwards bit order from what the problem would be, but it'll work.
        key: [
            0b1111111, // acedgfb  [3] seven bits -> 8
            0b0111110, // cdfbe    [8] overlaps with 9 -> 5
            0b1011011, // gcdfa    [9] -> 2
            0b1111010, // fbcad    [7] overlaps with 1 -> 3
            0b1101000, // dab      [1] three bits -> 7
            0b1111110, // cefabd   [4] overlaps with 4 -> 9
            0b0111111, // cdfgeb   [6] 6 bits -> 6
            0b1100110, // eafb     [2] four bits -> 4
            0b1111101, // cagedb   [5] overlaps with 1 and has 6 bits -> 0
            0b1100000, // ab       [0] two bits -> 1
        ],
        code: [
            0b0111110, // cdfeb
            0b1111010, // fcadb
            0b0111110, // cdfeb
            0b1111010, // cdbaf
        ],
    };
    assert_eq!(5353, decode(&line).unwrap());
}
