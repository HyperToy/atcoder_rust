use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        es: [((Usize1, Usize1), i64); m],
    }
    let mut g = vec![Vec::new(); n];
    for ((u, v), w) in es {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let g = g;

    let mut dist = vec![std::i64::MAX; n];
    let mut q = BinaryHeap::new();
    let start = 0;
    dist[start] = a[start];
    q.push((Reverse(dist[start]), start));
    while let Some((Reverse(d), u)) = q.pop() {
        if dist[u] < d {
            continue;
        }
        for &(v, w) in &g[u] {
            let nd = d + w + a[v];
            if dist[v] <= nd {
                continue;
            }
            dist[v] = nd;
            q.push((Reverse(nd), v));
        }
    }
    println!("{}", dist.into_iter().skip(1).join(" "));
}
