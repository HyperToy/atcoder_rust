use proconio::{marker::Usize1, *};
use std::collections::VecDeque;

fn main() {
    input! {
        n1: usize, n2: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }
    let n = n1 + n2;
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }

    let mut dist = vec![1_000_000_000; n];
    let mut q = VecDeque::new();
    let mut bfs = |start: usize| {
        dist[start] = 0;
        q.push_back(start);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for v in g[u].clone() {
                if dist[v] > dist[u] {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }
    };
    bfs(0);
    bfs(n - 1);

    let max_dist1 = dist[0..n1].iter().max().unwrap();
    let max_dist2 = dist[n1..n].iter().max().unwrap();
    println!("{}", max_dist1 + max_dist2 + 1);
}
