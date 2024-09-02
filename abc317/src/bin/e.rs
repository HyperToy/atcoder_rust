use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h],
    }
    let find = |c| {
        (0..h * w)
            .map(|a| (a / w, a % w))
            .find(|&(i, j)| grid[i][j] == c)
            .unwrap()
    };
    let start = find('S');
    let goal = find('G');
    let grid = convert(grid);
    // eprintln!("{:?}", grid);

    let mut dist = vec![vec![None; w]; h];
    let mut q = VecDeque::new();
    dist[start.0][start.1] = Some(0);
    q.push_back(start);
    while let Some((i, j)) = q.pop_front() {
        let d = dist[i][j].unwrap();
        for (ni, nj) in neighbors(i, j, h, w) {
            if grid[ni][nj] != '.' {
                continue;
            }
            if dist[ni][nj].is_some_and(|nd| nd <= d + 1) {
                continue;
            }
            dist[ni][nj] = Some(d + 1);
            q.push_back((ni, nj));
        }
    }
    println!(
        "{}",
        if let Some(d) = dist[goal.0][goal.1] {
            d
        } else {
            -1
        }
    );
}
fn convert(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut state = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '>' {
                state[i][j] |= 1 << 0;
            }
            if grid[i][j] == 'v' {
                state[i][j] |= 1 << 1;
            }
            if ['.', 'S', 'G'].contains(&grid[i][j]) {
                if j > 0 && state[i][j - 1] >> 0 & 1 == 1 {
                    state[i][j] |= 1 << 0;
                }
                if i > 0 && state[i - 1][j] >> 1 & 1 == 1 {
                    state[i][j] |= 1 << 1;
                }
            }
        }
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if grid[i][j] == '<' {
                state[i][j] |= 1 << 2;
            }
            if grid[i][j] == '^' {
                state[i][j] |= 1 << 3;
            }
            if ['.', 'S', 'G'].contains(&grid[i][j]) {
                if j < w - 1 && state[i][j + 1] >> 2 & 1 == 1 {
                    state[i][j] |= 1 << 2;
                }
                if i < h - 1 && state[i + 1][j] >> 3 & 1 == 1 {
                    state[i][j] |= 1 << 3;
                }
            }
        }
    }
    let mut res = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' || state[i][j] > 0 {
                res[i][j] = '#';
            }
        }
    }
    res
}
fn neighbors(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|&(dy, dx)| (i as isize + dy, j as isize + dx))
        .filter_map(|(ni, nj)| {
            if inside(ni, nj, h, w) {
                Some((ni as usize, nj as usize))
            } else {
                None
            }
        })
        .collect_vec()
}
fn inside(i: isize, j: isize, h: usize, w: usize) -> bool {
    0 <= i && i < h as isize && 0 <= j && j < w as isize
}
