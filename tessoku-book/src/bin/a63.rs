use proconio::{marker::Usize1, *};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }
    let mut dist = vec![std::i32::MAX; n];
    let mut queue = VecDeque::new();
    let start = 0;
    dist[start] = 0;
    queue.push_back(start);
    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for &v in &g[u] {
            if dist[v] > dist[u] {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    for i in 0..n {
        println!("{}", if dist[i] < std::i32::MAX { dist[i] } else { -1 });
    }
}
