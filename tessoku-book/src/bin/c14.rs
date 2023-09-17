use proconio::{marker::Usize1, *};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

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

    // bfs
    let mut seen = vec![false; n];
    let mut q = VecDeque::new();
    let goal = n - 1;
    seen[goal] = true;
    q.push_back(goal);
    let mut answer = 1;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for (v, w) in g[u].clone() {
            if seen[v] {
                continue;
            }
            if dist[v] == dist[u] - w {
                q.push_back(v);
                seen[v] = true;
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
