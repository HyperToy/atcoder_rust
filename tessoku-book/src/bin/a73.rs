use proconio::{marker::Usize1, *};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1, i64, i64); m],
    }
    let value = 1_000_000; // 補正値

    // dijkstra
    let mut g = vec![Vec::new(); n];
    for (u, v, c, d) in es {
        g[u].push((v, c * value - d));
        g[v].push((u, c * value - d));
    }
    let mut dist = vec![std::i64::MAX; n];
    let mut queue = BinaryHeap::new();
    let start = 0;
    dist[start] = 0;
    queue.push((Reverse(dist[start]), start));
    while !queue.is_empty() {
        let u = queue.pop().unwrap().1;
        for (v, w) in g[u].clone() {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                queue.push((Reverse(dist[v]), v));
            }
        }
    }
    let answer_dist = (dist[n - 1] + value - 1) / value;
    let tree_count = answer_dist * value - dist[n - 1];
    println!("{} {}", answer_dist, tree_count);
}
