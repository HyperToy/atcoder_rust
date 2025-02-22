use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashSet};

fn main() {
    input! {
        n: usize, m: usize, l: usize,
        a: [i32; n],
        b: [i32; m],
        cd: [(Usize1, Usize1); l],
    }
    let cd = cd.into_iter().collect::<HashSet<_>>();
    let a = a
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, v)| v)
        .rev()
        .collect_vec();
    let b = b
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, v)| v)
        .rev()
        .collect_vec();
    let mut q = BinaryHeap::new();
    for i in 0..n {
        q.push((a[i].1 + b[0].1, (i, 0)));
    }
    while let Some((v, (i, j))) = q.pop() {
        if !cd.contains(&(a[i].0, b[j].0)) {
            println!("{}", v);
            break;
        }
        if j + 1 < m {
            q.push((a[i].1 + b[j + 1].1, (i, j + 1)));
        }
    }
}
