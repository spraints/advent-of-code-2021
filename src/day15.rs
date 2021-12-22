use super::common::read_lines::read_lines;
use std::collections::HashMap;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let grid = read_lines(r)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    println!("part 1: {}", least_cost(&grid));
    let grid = expand(grid);
    println!("part 2: {}", least_cost(&grid));
}

fn expand(orig: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut res = Vec::with_capacity(orig.len() * 5);
    for rr in 0..5 {
        for orig_row in orig.iter() {
            let mut row = Vec::with_capacity(orig_row.len() * 5);
            for cr in 0..5 {
                for orig_val in orig_row.iter() {
                    row.push(1 + (orig_val + cr + rr - 1) % 9);
                }
            }
            res.push(row);
        }
    }
    res
}

const INF: u32 = 999_999_999;

const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn least_cost(grid: &[Vec<u32>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let (dest_r, dest_c) = (rows - 1, cols - 1);

    let mut visited = grid
        .iter()
        .map(|r| r.iter().map(|_| false).collect())
        .collect::<Vec<Vec<bool>>>();
    let mut tentative_distance = grid
        .iter()
        .map(|r| r.iter().map(|_| INF).collect())
        .collect::<Vec<Vec<u32>>>();

    let (mut r, mut c) = (0, 0);
    tentative_distance[r][c] = 0;
    let mut dist = 0;

    let mut lowest_unvisited_cost = 0;
    let mut by_cost = HashMap::new();

    loop {
        for (dr, dc) in NEIGHBORS {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr < 0 || nc < 0 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if nr >= rows || nc >= cols || visited[nr][nc] {
                continue;
            }
            let new_dist = dist + grid[nr][nc];
            if new_dist < tentative_distance[nr][nc] {
                tentative_distance[nr][nc] = new_dist;
                let e = by_cost.entry(new_dist).or_insert_with(Vec::new);
                e.push((nr, nc));
                if new_dist < lowest_unvisited_cost {
                    println!("did not expect this to go backwards?");
                    lowest_unvisited_cost = new_dist;
                }
            }
        }

        visited[r][c] = true;

        if visited[dest_r][dest_c] {
            return tentative_distance[dest_r][dest_c];
        }

        let next_point = get_next_node(&mut lowest_unvisited_cost, &mut by_cost, &visited);
        r = next_point.0;
        c = next_point.1;
        dist = tentative_distance[r][c];
    }
}

fn get_next_node(
    lowest_unvisited_cost: &mut u32,
    by_cost: &mut HashMap<u32, Vec<(usize, usize)>>,
    visited: &[Vec<bool>],
) -> (usize, usize) {
    loop {
        assert!(*lowest_unvisited_cost < 10_000);
        match by_cost.get_mut(lowest_unvisited_cost) {
            None => *lowest_unvisited_cost += 1,
            Some(points) => match points.pop() {
                None => *lowest_unvisited_cost += 1,
                Some((r, c)) => {
                    if !visited[r][c] {
                        return (r, c);
                    }
                }
            },
        };
    }
}
