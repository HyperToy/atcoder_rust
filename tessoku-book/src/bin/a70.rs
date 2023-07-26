use proconio::{marker::Usize1, *};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        ops: [(Usize1, Usize1, Usize1); m],
    }
    let mut init = 0;
    for i in 0..n {
        init |= a[i] << i;
    }
    let ops = ops
        .iter()
        .map(|(x, y, z)| (1usize << x) | (1usize << y) | (1usize << z))
        .collect::<Vec<usize>>();
    let mut g = vec![Vec::new(); 1 << n];
    for i in 0..(1 << n) {
        for j in 0..m {
            g[i].push(i ^ ops[j]);
        }
    }
    let mut dist = vec![std::i32::MAX; 1 << n];
    let mut queue = VecDeque::new();
    dist[init] = 0;
    queue.push_back(init);
    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for v in g[u].clone() {
            if dist[v] > dist[u] + 1 {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    println!(
        "{}",
        if dist[(1 << n) - 1] < std::i32::MAX {
            dist[(1 << n) - 1]
        } else {
            -1
        }
    );
}
