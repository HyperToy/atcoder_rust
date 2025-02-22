use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        stages: [(i64, i64, Usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n]; // (to, cost)
    for (i, (a, b, x)) in stages.into_iter().enumerate() {
        g[i].push((i + 1, a));
        g[i].push((x, b));
    }
    let mut dist = vec![std::i64::MAX; n];
    dist[0] = 0;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(dist[0]), 0));
    while !queue.is_empty() {
        let (Reverse(d), u) = queue.pop().unwrap();
        if d > dist[u] {
            continue;
        }
        for (v, w) in &g[u] {
            if dist[*v] > dist[u] + w {
                dist[*v] = dist[u] + w;
                queue.push((Reverse(dist[*v]), *v));
            }
        }
    }
    println!("{}", dist[n - 1]);
}
