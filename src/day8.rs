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
