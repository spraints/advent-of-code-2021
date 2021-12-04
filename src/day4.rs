use super::common::read_sections::read_sections;
use std::collections::HashMap;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut sections = read_sections(r);
    let order = sections.next().unwrap().remove(0);
    let mut boards = vec![];
    for section in sections {
        boards.push(parse_board(section));
    }
    let mut first_winner = None;
    let mut last_winner = None;
    for n in order.split(',') {
        let n = n.parse().unwrap();
        let (ww, ll): (Vec<Board>, Vec<Board>) = boards
            .into_iter()
            .map(|board| board.play(n))
            .partition(|board| board.is_winner());
        for w in ww.into_iter() {
            let score = n * w.unused();
            last_winner = Some(score);
            if first_winner.is_none() {
                first_winner = Some(score);
            }
        }
        boards = ll;
    }
    println!("part 1: {}", first_winner.unwrap());
    println!("part 2: {}", last_winner.unwrap());
}

struct Board {
    rows: [u8; 5],
    cols: [u8; 5],
    tiles: HashMap<u32, (usize, usize)>,
}

impl Board {
    fn play(mut self, n: u32) -> Self {
        match self.tiles.remove(&n) {
            None => self,
            Some((i, j)) => {
                let (mut rows, mut cols, tiles) = (self.rows, self.cols, self.tiles);
                rows[i] += 1;
                cols[j] += 1;
                Self { rows, cols, tiles }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for r in self.rows {
            if r == 5 {
                return true;
            }
        }
        for c in self.cols {
            if c == 5 {
                return true;
            }
        }
        false
    }

    fn unused(&self) -> u32 {
        self.tiles.keys().sum()
    }
}

fn parse_board(raw: Vec<String>) -> Board {
    let mut tiles = HashMap::new();
    for (i, row) in raw.into_iter().enumerate() {
        for (j, n) in row.trim().split_whitespace().enumerate() {
            let n: u32 = n.parse().unwrap();
            tiles.insert(n, (i, j));
        }
    }
    Board {
        rows: [0; 5],
        cols: [0; 5],
        tiles,
    }
}
