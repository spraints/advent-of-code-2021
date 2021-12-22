use super::common::read_lines::read_lines;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Read;

struct Universe {
    positions: [u16; 2],
    scores: [u16; 2],
    roll_sum: u16,
}

type UniverseCounts = HashMap<Universe, usize>;

pub fn run<R: Read>(r: R) {
    let start_pos = make_start_pos(read_lines(r).map(|line| parse_pos(&line)));
    println!("part 1: {}", part1(start_pos.clone()));
    println!("part 2: {}", part2(start_pos.clone()));
}

fn part1(mut pos: [u16; 2]) -> usize {
    let mut rolls = 0;
    let mut scores = [0; 2];
    let mut player = 0;
    loop {
        let roll_sum = droll(&mut rolls) + droll(&mut rolls) + droll(&mut rolls);
        pos[player] = make_move(pos[player], roll_sum);
        scores[player] += pos[player];
        if scores[player] > 999 {
            return (rolls as usize) * (scores[player - 1] as usize);
        }
        player = 1 - player;
    }
}

fn droll(rolls: &mut usize) -> u16 {
    let roll = 1 + ((*rolls % 100) as u16);
    *rolls += 1;
    roll
}

fn part2(_: [u16; 2]) -> usize {
    0
}

fn make_move(pos: u16, rolled: u16) -> u16 {
    1 + (pos + rolled - 1) % 10
}

fn parse_pos(line: &str) -> u16 {
    let c = line.chars().last().unwrap();
    c.to_digit(10).unwrap().try_into().unwrap()
}

fn make_start_pos<I: Iterator<Item = u16>>(pos: I) -> [u16; 2] {
    let mut res = [0; 2];
    for (i, pos) in pos.enumerate() {
        res[i] = pos;
    }
    assert_ne!(res[1], 0);
    res
}
