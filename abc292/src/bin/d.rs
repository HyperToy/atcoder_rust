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
        g[v].push(u);
    }
    let mut seen = vec![false; n];
    let mut q = VecDeque::new();
    let mut ok = true;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        seen[i] = true;
        let mut count_v = 0;
        let mut count_e = 0;
        q.push_back(i);
        while !q.is_empty() {
            count_v += 1;
            let u = q.pop_front().unwrap();
            for v in &g[u] {
                count_e += 1;
                if seen[*v] {
                    continue;
                }
                seen[*v] = true;
                q.push_back(*v);
            }
        }
        ok &= count_v == count_e / 2;
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
