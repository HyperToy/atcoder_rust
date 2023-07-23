use proconio::{marker::Usize1, *};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1, i32); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v, w) in es {
        g[u].push((v, w));
        g[v].push((u, w));
    }

    let mut dist = vec![std::i32::MAX; n];
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
    for i in 0..n {
        println!("{}", if dist[i] < std::i32::MAX { dist[i] } else { -1 });
    }
}
