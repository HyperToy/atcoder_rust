use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }
    // 次数 1 の頂点を探す
    let start = g
        .iter()
        .enumerate()
        .find_map(|(i, to)| if to.len() == 1 { Some(i) } else { None })
        .unwrap();
    let mut vs = vec![Kind::Uncertain; n];
    let mut q = VecDeque::new();
    vs[start] = Kind::Leaf;
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        match vs[u] {
            Kind::Center => {
                for &v in &g[u] {
                    if vs[v] != Kind::Uncertain {
                        continue;
                    }
                    vs[v] = Kind::Leaf;
                    q.push_back(v);
                }
            }
            Kind::Leaf => {
                if g[u].len() == 1 {
                    let v = g[u][0];
                    if vs[v] == Kind::Uncertain {
                        vs[v] = Kind::Center;
                        q.push_back(v);
                    }
                } else {
                    assert_eq!(g[u].len(), 2);
                    let (v1, v2) = (g[u][0], g[u][1]);
                    match (vs[v1], vs[v2]) {
                        (Kind::Center, Kind::Uncertain) => {
                            vs[v2] = Kind::Leaf;
                            q.push_back(v2);
                        }
                        (Kind::Leaf, Kind::Uncertain) => {
                            vs[v2] = Kind::Center;
                            q.push_back(v2);
                        }
                        (Kind::Uncertain, Kind::Center) => {
                            vs[v1] = Kind::Leaf;
                            q.push_back(v1);
                        }
                        (Kind::Uncertain, Kind::Leaf) => {
                            vs[v1] = Kind::Center;
                            q.push_back(v1);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Kind::Uncertain => unreachable!(),
        }
    }
    println!(
        "{}",
        vs.into_iter()
            .enumerate()
            .filter_map(|(v, k)| {
                match k {
                    Kind::Center => Some(g[v].len()),
                    Kind::Leaf => None,
                    Kind::Uncertain => unreachable!(),
                }
            })
            .sorted()
            .join(" ")
    );
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Uncertain,
    Center,
    Leaf,
}
