use proconio::{input, marker::Usize1};
use std::collections::{HashMap, VecDeque};

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
    let group = bipartite(&g);
    if group == None {
        println!("0");
        return;
    }
    let group = group.unwrap();
    let mut group_count = HashMap::new();
    for i in 0..n {
        *(group_count.entry(&group[i]).or_insert(0i64)) += 1;
    }
    let bad_edge = group_count
        .into_iter()
        .map(|(_, v)| v * (v - 1) / 2)
        .sum::<i64>();
    let all_edge = n * (n - 1) / 2;
    println!("{}", all_edge - m - bad_edge as usize);
}

fn bipartite(g: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = g.len();
    let mut seen = vec![false; n];
    let mut group = vec![std::usize::MAX; n];
    let mut q = VecDeque::new();
    let mut id = 0;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        // 2 * id, 2 * id + 1 を 二部グラフを構成するグループとする
        let id_sum = 4 * id + 1;
        seen[i] = true;
        group[i] = 2 * id;
        q.push_back(i);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for &v in &g[u] {
                if group[v] == group[u] {
                    return None;
                }
                if seen[v] {
                    continue;
                }
                seen[v] = true;
                group[v] = id_sum - group[u];
                q.push_back(v);
            }
        }
        id += 1;
    }
    Some(group)
}
