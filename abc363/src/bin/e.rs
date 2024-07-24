use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize, w: usize, y: usize,
        a: [[Usize1; w]; h],
    }
    let mut q = vec![Vec::new(); y];
    let mut inland = vec![vec![true; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                if a[i][j] < y {
                    q[a[i][j]].push((i, j));
                }
                inland[i][j] = false;
            }
        }
    }
    let mut answers = Vec::new();
    let mut answer = h * w;
    for k in 0..y {
        while let Some((i, j)) = q[k].pop() {
            answer -= 1;
            for (ni, nj) in neighbors(i, j, h, w) {
                if inland[ni][nj] {
                    if a[ni][nj] < y {
                        q[a[ni][nj].max(k)].push((ni, nj));
                    }
                    inland[ni][nj] = false;
                }
            }
        }
        answers.push(answer);
    }
    println!("{}", answers.iter().join("\n"));
}
fn neighbors(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    if i > 0 {
        res.push((i - 1, j));
    }
    if j > 0 {
        res.push((i, j - 1));
    }
    if i < h - 1 {
        res.push((i + 1, j));
    }
    if j < w - 1 {
        res.push((i, j + 1));
    }
    res
}
