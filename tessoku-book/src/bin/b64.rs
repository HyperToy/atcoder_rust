use itertools::Itertools;
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

    // dijkstra
    let mut dist = vec![std::i32::MAX; n];
    let mut q = BinaryHeap::new();
    let start = 0;
    dist[start] = 0;
    q.push((Reverse(dist[start]), start));

    while !q.is_empty() {
        let u = q.pop().unwrap().1;
        for (v, w) in g[u].clone() {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                q.push((Reverse(dist[v]), v));
            }
        }
    }

    let mut answer = Vec::new();
    let goal = n - 1;
    let mut pos = goal;
    answer.push(pos);
    while pos != start {
        for (v, w) in g[pos].clone() {
            if dist[pos] - w == dist[v] {
                answer.push(v);
                pos = v;
                break;
            }
        }
    }
    println!("{}", answer.iter().rev().map(|x| x + 1).join(" "));
}
