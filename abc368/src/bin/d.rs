use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, k: usize,
        es: [(Usize1, Usize1); n - 1],
        vs: [Usize1; k],
    }
    let mut g = vec![BTreeSet::new(); n];
    for &(u, v) in &es {
        g[u].insert(v);
        g[v].insert(u);
    }
    let mut target = vec![false; n];
    for v in vs {
        target[v] = true;
    }
    let mut dim = vec![0; n];
    for (u, v) in es {
        dim[u] += 1;
        dim[v] += 1;
    }
    let mut ones = vec![];
    for i in 0..n {
        if dim[i] == 1 {
            ones.push(i);
        }
    }
    let mut need = vec![true; n];
    while let Some(v) = ones.pop() {
        if target[v] {
            continue;
        }
        need[v] = false;
        dim[v] -= 1;
        let u = g[v].pop_first().unwrap();
        g[u].remove(&v);
        dim[u] -= 1;
        if dim[u] == 1 {
            ones.push(u);
        }
    }
    println!("{}", need.iter().filter(|&&b| b).count());
}
