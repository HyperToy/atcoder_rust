use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        abxy: [(Usize1, Usize1, i64, i64); m],
    }
    let mut g = vec![Vec::new(); n];
    for (a, b, x, y) in abxy {
        g[a].push((b, (x, y)));
        g[b].push((a, (-x, -y)));
    }
    let mut pos = vec![None; n];
    let mut queue = VecDeque::new();
    pos[0] = Some((0, 0));
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        let (nx, ny) = pos[u].unwrap();
        for (v, (x, y)) in g[u].clone() {
            if let Some(_) = pos[v] {
                continue;
            } else {
                pos[v] = Some((nx + x, ny + y));
                queue.push_back(v);
            }
        }
    }
    println!(
        "{}",
        pos.iter()
            .map(|p| if let Some((x, y)) = p {
                format!("{} {}", x, y)
            } else {
                "undecidable".to_string()
            })
            .join(" ")
    );
}
