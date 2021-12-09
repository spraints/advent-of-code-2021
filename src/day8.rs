use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let lines: Vec<Line> = read_lines(r)
        .map(|line| parse_line(&line))
        .collect::<Result<Vec<Line>, _>>()
        .unwrap();

    println!("part 1: {}", count_simple(&lines));
    let nums = lines.iter().map(|line| decode(line));
    println!("part 2: {}", nums.sum::<u32>());
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

fn decode(line: &Line) -> u32 {
    1
}
