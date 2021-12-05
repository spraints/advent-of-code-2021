use super::common::read_lines::read_lines;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let (diag, other) = read_lines(r).map(parse_line).partition(is_diag);
    let mut map = HashMap::new();
    fill(&mut map, other);
    println!("part 1: {}", score(&map));
    fill(&mut map, diag);
    println!("part 2: {}", score(&map));
}

fn fill(map: &mut HashMap<Point, usize>, points: Vec<Line>) {
    for (p1, p2) in points {
        let step = (step(p1.0, p2.0), step(p1.1, p2.1));
        let mut p = p1;
        loop {
            let last = p == p2;
            let counter = map.entry(p).or_insert(0);
            *counter += 1;
            if last {
                break;
            }
            p = (p.0 + step.0, p.1 + step.1);
        }
    }
}

fn step(from: i32, to: i32) -> i32 {
    match from.cmp(&to) {
        Ordering::Greater => -1,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

fn score(map: &HashMap<Point, usize>) -> usize {
    map.iter().filter(|(_, count)| **count > 1).count()
}

type Point = (i32, i32);
type Line = (Point, Point);

fn parse_line(s: String) -> Line {
    let mut points = s.split(" -> ").map(|point| parse_point(point));
    let from = points.next().unwrap();
    let to = points.next().unwrap();
    (from, to)
}

fn parse_point(s: &str) -> Point {
    let mut coords = s.split(',');
    let x = coords.next().unwrap().parse().unwrap();
    let y = coords.next().unwrap().parse().unwrap();
    (x, y)
}

fn is_diag(line: &Line) -> bool {
    let ((x1, y1), (x2, y2)) = line;
    x1 != x2 && y1 != y2
}
