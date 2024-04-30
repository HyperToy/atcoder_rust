// todo
use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const DY: [isize; 4] = [0, 1, 0, -1];
const DX: [isize; 4] = [1, 0, -1, 0];
fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }
    let free = free(h, w, &s); // 自由に動けるマスか
    let mut answer = 1;
    let mut seen = vec![vec![false; w]; h];
    let mut q = VecDeque::new();
    let mut visited;
    for (i, j) in (0..h).cartesian_product(0..w) {
        if !free[i][j] || seen[i][j] {
            continue;
        }
        visited = vec![vec![false; w]; h];
        q.push_back((i, j));
        visited[i][j] = true;
        let mut now = 0;
        while !q.is_empty() {
            now += 1;
            let (y, x) = q.pop_front().unwrap();
            if !free[y][x] {
                continue;
            }
            seen[y][x] = true;
            for k in 0..4 {
                let ny = y as isize + DY[k];
                let nx = x as isize + DX[k];
                if !inside(ny, nx, h, w) {
                    continue;
                }
                let (ny, nx) = (ny as usize, nx as usize);
                if visited[ny][nx] {
                    continue;
                }
                q.push_back((ny, nx));
                visited[ny][nx] = true;
            }
        }
        answer = answer.max(now);
    }
    println!("{}", answer);
}
fn free(h: usize, w: usize, s: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut free = vec![vec![true; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        if s[i][j] == '#' {
            free[i][j] = false;
            continue;
        }
        for k in 0..4 {
            let ni = i as isize + DY[k];
            let nj = j as isize + DX[k];
            if !inside(ni, nj, h, w) {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if s[ni][nj] == '#' {
                free[i][j] = false;
                break;
            }
        }
    }
    free
}
fn inside(y: isize, x: isize, h: usize, w: usize) -> bool {
    0 <= y && y < h as isize && 0 <= x && x < w as isize
}
