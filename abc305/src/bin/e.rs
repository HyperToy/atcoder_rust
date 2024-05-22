use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, i32); k],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in ab {
        g[u].push(v);
        g[v].push(u);
    }
    let mut pq = BinaryHeap::new();
    let mut d = vec![std::i32::MIN; n];
    for (p, h) in ph {
        d[p] = h;
        pq.push((d[p], p));
    }
    while !pq.is_empty() {
        let (now, u) = pq.pop().unwrap();
        if now > d[u] {
            continue;
        }
        if now == 0 {
            continue;
        }
        for &v in &g[u] {
            if d[v] >= now - 1 {
                continue;
            }
            d[v] = now - 1;
            pq.push((d[v], v));
        }
    }
    let answer = d
        .into_iter()
        .enumerate()
        .filter_map(|(i, d)| if d > std::i32::MIN { Some(i + 1) } else { None })
        .collect::<Vec<_>>();
    println!("{}\n{}", answer.len(), answer.iter().join(" "));
}
