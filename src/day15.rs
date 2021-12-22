use super::common::read_lines::read_lines;
use std::io::Read;

pub fn run<R: Read>(r: R) {
    let grid = read_lines(r)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    println!("part 1: {}", least_cost(&grid));
}

const INF: u32 = 999_999_999;

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

    let (mut est_rows, mut est_cols) = (0, 0);

    loop {
        //println!("visiting {},{} (dist = {})", r, c, dist);

        if r + 1 < rows {
            update(&mut tentative_distance, &visited, &grid, (r + 1, c), dist);
        }
        if c + 1 < cols {
            update(&mut tentative_distance, &visited, &grid, (r, c + 1), dist);
        }
        if r > 0 {
            update(&mut tentative_distance, &visited, &grid, (r - 1, c), dist);
        }
        if c > 0 {
            update(&mut tentative_distance, &visited, &grid, (r, c - 1), dist);
        }
        /*
        if r + 1 < rows && !visited[r + 1][c] {
            let this_dist = dist + grid[r + 1][c];
            if tentative_distance[r + 1][c] > this_dist {
                println!(" ... update {},{} dist to {}", r + 1, c, this_dist);
                tentative_distance[r + 1][c] = this_dist;
            }
        }
        if c + 1 < cols && !visited[r][c + 1] {
            let this_dist = dist + grid[r][c + 1];
            if tentative_distance[r][c + 1] > this_dist {
                println!(" ... update {},{} dist to {}", r, c + 1, this_dist);
                tentative_distance[r][c + 1] = this_dist;
            }
        }
        */

        if r + 1 >= est_rows {
            est_rows = r + 2;
        }
        if c + 1 >= est_cols {
            est_cols = c + 2;
        }

        visited[r][c] = true;

        if visited[dest_r][dest_c] {
            /*
            let mut unvisited = 0;
            for i in 0..rows {
                for j in 0..cols {
                    if !visited[r][c] {
                        unvisited += 1;
                    }
                }
            }
            println!("done! {} unvisited nodes", unvisited);
            */

            return tentative_distance[dest_r][dest_c];
        }

        //println!("  valid space {},{}", est_rows, est_cols);
        let (mut next_r, mut next_c) = (0, 0);
        let mut next_dist = INF;
        for i in 0..est_rows {
            if i < rows {
                for j in 0..est_cols {
                    if j < cols && !visited[i][j] {
                        let this_dist = tentative_distance[i][j];
                        //println!("  should i visit {},{} (dist = {}) next?", i, j, this_dist);
                        if this_dist < next_dist {
                            next_dist = this_dist;
                            next_r = i;
                            next_c = j;
                        }
                    }
                }
            }
        }

        r = next_r;
        c = next_c;
        dist = next_dist;
    }
}

fn update(
    tentative_distance: &mut Vec<Vec<u32>>,
    visited: &[Vec<bool>],
    grid: &[Vec<u32>],
    coords: (usize, usize),
    dist: u32,
) {
    let (r, c) = coords;
    if !visited[r][c] {
        let new_dist = dist + grid[r][c];
        if new_dist < tentative_distance[r][c] {
            tentative_distance[r][c] = new_dist;
        }
    }
}
