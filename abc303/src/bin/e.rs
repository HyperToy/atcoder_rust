use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }
    // 次数 1 の頂点を探す
    let leaf = g
        .iter()
        .enumerate()
        .find_map(|(v, to)| if to.len() == 1 { Some(v) } else { None })
        .unwrap();
    let center = g[leaf][0];
    let mut dist = vec![std::i32::MAX; n];
    let mut q = VecDeque::new();
    dist[center] = 0;
    q.push_back(center);
    while let Some(u) = q.pop_front() {
        for &v in &g[u] {
            if dist[v] > dist[u] + 1 {
                dist[v] = dist[u] + 1;
                q.push_back(v)
            }
        }
    }
    println!(
        "{}",
        dist.into_iter()
            .enumerate()
            .filter_map(|(v, d)| {
                if d % 3 == 0 {
                    Some(g[v].len())
                } else {
                    None
                }
            })
            .sorted()
            .join(" ")
    );
}
