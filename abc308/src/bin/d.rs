use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let n = h * w;
    let mut g = vec![vec![]; n];
    for i in 0..h {
        for j in 0..w {
            for (ni, nj) in neighbors(i, j, h, w) {
                if check(s[i][j], s[ni][nj]) {
                    g[i * w + j].push(ni * w + nj);
                }
            }
        }
    }
    let start = 0;
    let goal = n - 1;
    let mut dist = vec![std::u32::MAX; n];
    let mut q = VecDeque::new();
    dist[start] = 0;
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        for &v in &g[u] {
            if dist[v] <= dist[u] + 1 {
                continue;
            }
            dist[v] = dist[u] + 1;
            q.push_back(v);
        }
    }
    println!(
        "{}",
        if dist[goal] < std::u32::MAX {
            "Yes"
        } else {
            "No"
        }
    );
}
fn neighbors(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|&(dy, dx)| (i as isize + dy, j as isize + dx))
        .filter_map(|(ni, nj)| {
            if 0 <= ni && ni < h as isize && 0 <= nj && nj < w as isize {
                Some((ni as usize, nj as usize))
            } else {
                None
            }
        })
        .collect_vec()
}
fn check(u: char, v: char) -> bool {
    match (u, v) {
        ('s', 'n') | ('n', 'u') | ('u', 'k') | ('k', 'e') | ('e', 's') => true,
        _ => false,
    }
}
