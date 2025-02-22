use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, u64); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    let sum = abc.iter().map(|(_, _, c)| c).sum::<u64>();
    for (u, v, w) in abc {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let d = diameter(&g);
    println!("{}", 2 * sum - d);
}
fn diameter(g: &Vec<Vec<(usize, u64)>>) -> u64 {
    let dist = bfs(g, 0);
    let v = dist
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, d)| d)
        .last()
        .unwrap()
        .0;
    let dist = bfs(g, v);
    dist.into_iter().max().unwrap()
}
fn bfs(g: &Vec<Vec<(usize, u64)>>, start: usize) -> Vec<u64> {
    let n = g.len();
    let mut dist = vec![std::u64::MAX; n];
    let mut q = VecDeque::new();
    dist[start] = 0;
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        for &(v, w) in &g[u] {
            if dist[v] < dist[u] + w {
                continue;
            }
            dist[v] = dist[u] + w;
            q.push_back(v);
        }
    }
    dist
}
