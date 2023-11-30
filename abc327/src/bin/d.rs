use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        (n, m): (usize, usize),
        a: [Usize1; m],
        b: [Usize1; m],
    }
    let mut g = vec![Vec::new(); n];
    for i in 0..m {
        g[a[i]].push(b[i]);
        g[b[i]].push(a[i]);
    }
    // 二部グラフの判定に帰着
    let mut ok = true;
    let mut color = vec![-1; n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        if color[i] != -1 {
            continue;
        }
        queue.push_back(i);
        color[i] = 0;
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            for v in g[u].clone() {
                if color[v] == color[u] {
                    ok = false;
                    continue;
                }
                if color[v] != -1 {
                    continue;
                }
                queue.push_back(v);
                color[v] = 1 - color[u];
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
