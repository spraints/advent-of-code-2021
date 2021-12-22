use super::common::read_sections::read_sections;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let mut sections = read_sections(r);
    let coords = sections.next().unwrap().into_iter().map(parse_coord);
    let mut instructions = sections.next().unwrap().into_iter().map(parse_inst);

    let mut coords = process(coords, instructions.next().unwrap());
    coords.sort_unstable();
    coords.dedup();
    println!("part 1: {}", coords.len());

    for inst in instructions {
        coords = process(coords.into_iter(), inst);
    }
    let mut grid = vec![];
    for coord in coords.into_iter() {
        let x = coord.x;
        let y = coord.y;
        if grid.len() <= y {
            grid.resize_with(y + 1, Vec::new);
        }
        let row = grid.get_mut(y).unwrap();
        if row.len() <= x {
            row.resize(x + 1, ' ');
        }
        row[x] = '#';
    }
    println!("part 2:");
    for row in grid.into_iter() {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}

fn process<C: Iterator<Item = Coord>>(coords: C, inst: Fold) -> Vec<Coord> {
    coords.map(|coord| inst.fold(coord)).collect()
}

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse_coord(line: String) -> Coord {
    let (a, b) = line.split_once(',').unwrap();
    Coord {
        x: a.parse().unwrap(),
        y: b.parse().unwrap(),
    }
}

enum Fold {
    Y(usize),
    X(usize),
}

fn parse_inst(line: String) -> Fold {
    let s = line.strip_prefix("fold along ").unwrap();
    let (dir, pos) = s.split_once('=').unwrap();
    match dir {
        "x" => Fold::X(pos.parse().unwrap()),
        "y" => Fold::Y(pos.parse().unwrap()),
        _ => panic!("can't parse {}", line),
    }
}

impl Fold {
    fn fold(&self, coord: Coord) -> Coord {
        match self {
            Self::X(x) => {
                if coord.x <= *x {
                    coord
                } else {
                    Coord {
                        x: 2 * *x - coord.x,
                        y: coord.y,
                    }
                }
            }
            Self::Y(y) => {
                if coord.y <= *y {
                    coord
                } else {
                    Coord {
                        x: coord.x,
                        y: 2 * *y - coord.y,
                    }
                }
            }
        }
    }
}

#[test]
fn test_fold() {
    assert_eq!(Coord { x: 0, y: 0 }, Fold::X(4).fold(Coord { x: 0, y: 0 }));
    assert_eq!(Coord { x: 1, y: 0 }, Fold::X(4).fold(Coord { x: 1, y: 0 }));
    assert_eq!(Coord { x: 2, y: 0 }, Fold::X(4).fold(Coord { x: 2, y: 0 }));
    assert_eq!(Coord { x: 3, y: 0 }, Fold::X(4).fold(Coord { x: 3, y: 0 }));
    assert_eq!(Coord { x: 3, y: 0 }, Fold::X(4).fold(Coord { x: 5, y: 0 }));
    assert_eq!(Coord { x: 2, y: 0 }, Fold::X(4).fold(Coord { x: 6, y: 0 }));
    assert_eq!(Coord { x: 1, y: 0 }, Fold::X(4).fold(Coord { x: 7, y: 0 }));
    assert_eq!(Coord { x: 0, y: 0 }, Fold::X(4).fold(Coord { x: 8, y: 0 }));

    assert_eq!(Coord { x: 0, y: 0 }, Fold::Y(4).fold(Coord { x: 0, y: 0 }));
    assert_eq!(Coord { x: 0, y: 1 }, Fold::Y(4).fold(Coord { x: 0, y: 1 }));
    assert_eq!(Coord { x: 0, y: 2 }, Fold::Y(4).fold(Coord { x: 0, y: 2 }));
    assert_eq!(Coord { x: 0, y: 3 }, Fold::Y(4).fold(Coord { x: 0, y: 3 }));
    assert_eq!(Coord { x: 0, y: 3 }, Fold::Y(4).fold(Coord { x: 0, y: 5 }));
    assert_eq!(Coord { x: 0, y: 2 }, Fold::Y(4).fold(Coord { x: 0, y: 6 }));
    assert_eq!(Coord { x: 0, y: 1 }, Fold::Y(4).fold(Coord { x: 0, y: 7 }));
    assert_eq!(Coord { x: 0, y: 0 }, Fold::Y(4).fold(Coord { x: 0, y: 8 }));
}
