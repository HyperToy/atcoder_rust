use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::{HashSet, VecDeque};

const DY: [isize; 4] = [0, 1, 0, -1];
const DX: [isize; 4] = [1, 0, -1, 0];
fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }
    let statuses = cell_statuses(h, w, &s);
    let mut answer = 1;
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        if statuses[i][j] != CellStatus::Free || visited[i][j] {
            continue;
        }
        q.push_back((i, j));
        visited[i][j] = true;
        let mut now = 0;
        let mut bound_cells = HashSet::new();
        while let Some((y, x)) = q.pop_front() {
            match statuses[y][x] {
                CellStatus::Magnet => (),
                CellStatus::Bound => {
                    now += 1;
                }
                CellStatus::Free => {
                    now += 1;
                    for (ny, nx) in neighbors(y, x, h, w) {
                        if visited[ny][nx] {
                            continue;
                        }
                        q.push_back((ny, nx));
                        visited[ny][nx] = true;
                        if statuses[ny][nx] == CellStatus::Bound {
                            bound_cells.insert((ny, nx));
                        }
                    }
                }
            }
        }
        answer = answer.max(now);
        for (y, x) in bound_cells {
            visited[y][x] = false;
        }
    }
    println!("{}", answer);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellStatus {
    Magnet,
    Bound,
    Free,
}
fn cell_statuses(h: usize, w: usize, s: &Vec<Vec<char>>) -> Vec<Vec<CellStatus>> {
    let mut res = vec![vec![CellStatus::Free; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        if s[i][j] == '#' {
            res[i][j] = CellStatus::Magnet;
            continue;
        }
        for (ni, nj) in neighbors(i, j, h, w) {
            if s[ni][nj] == '#' {
                res[i][j] = CellStatus::Bound;
                break;
            }
        }
    }
    res
}
fn neighbors(y: usize, x: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for k in 0..4 {
        let ny = y as isize + DY[k];
        let nx = x as isize + DX[k];
        if !inside(ny, nx, h, w) {
            continue;
        }
        let (ny, nx) = (ny as usize, nx as usize);
        res.push((ny, nx));
    }
    res
}
fn inside(y: isize, x: isize, h: usize, w: usize) -> bool {
    0 <= y && y < h as isize && 0 <= x && x < w as isize
}
