use super::common::read_lines::read_lines;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Read;

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

#[derive(Hash, Eq, PartialEq)]
struct Universe {
    positions: [u16; 2],
    scores: [u16; 2],
    roll_sum: u16,
}

type UniverseCounts = HashMap<Universe, usize>;

fn part2(start_pos: [u16; 2]) -> usize {
    let mut wins = [0_usize, 0];

    let mut universes = HashMap::new();
    universes.insert(
        Universe {
            positions: start_pos,
            scores: [0, 0],
            roll_sum: 0,
        },
        1,
    );

    let mut player = 0;
    while universes.len() > 0 {
        let (won, new_universes) = play_turn(universes, player);
        wins[player] += won;
        universes = new_universes;
        player = 1 - player;
    }

    if wins[0] > wins[1] {
        wins[0]
    } else {
        wins[1]
    }
}

fn play_turn(mut universes: UniverseCounts, player: usize) -> (usize, UniverseCounts) {
    // Roll three times...
    for _ in 0..3 {
        let mut new_universes = HashMap::with_capacity(universes.len() * 3);
        // Split the universe for each possible roll...
        for roll in 1..=3 {
            for (universe, count) in universes.iter() {
                let ru = do_roll(&universe, roll);
                let e = new_universes.entry(ru).or_insert(0);
                *e += count;
            }
        }
        universes = new_universes;
    }
    // Update the positions and scores, etc.
    let mut new_universes = HashMap::with_capacity(universes.len());
    let mut wins = 0;
    for (mut universe, count) in universes.into_iter() {
        move_and_score(&mut universe, player);
        if universe.scores[player] > 20 {
            wins += count
        } else {
            let e = new_universes.entry(universe).or_insert(0);
            *e += count;
        }
    }
    (wins, new_universes)
}

fn move_and_score(universe: &mut Universe, player: usize) {
    universe.positions[player] = make_move(universe.positions[player], universe.roll_sum);
    universe.scores[player] += universe.positions[player];
    universe.roll_sum = 0;
}

fn do_roll(universe: &Universe, roll: u16) -> Universe {
    Universe {
        roll_sum: universe.roll_sum + roll,
        positions: universe.positions,
        scores: universe.scores,
    }
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
