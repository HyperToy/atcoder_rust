use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
    }
    let mut answer = 0;
    for i in 0..n {
        let mut q = VecDeque::new();
        let mut seen = vec![false; n];
        q.push_back(i);
        seen[i] = true;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for v in g[u].clone() {
                if seen[v] {
                    continue;
                }
                seen[v] = true;
                q.push_back(v);
                answer += 1;
            }
        }
    }
    answer -= m;
    println!("{}", answer);
}
