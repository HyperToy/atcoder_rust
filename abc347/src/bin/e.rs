use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, q: usize,
        xs: [Usize1; q],
    }
    let mut s = HashSet::new();
    let mut a = vec![0; n];
    let mut size_sum = vec![0; q + 1];
    let mut appear = vec![None; n];
    for (i, x) in xs.into_iter().enumerate() {
        if s.contains(&x) {
            a[x] += size_sum[i] - size_sum[appear[x].unwrap()];
            s.remove(&x);
            appear[x] = None;
        } else {
            appear[x] = Some(i);
            s.insert(x);
        }
        size_sum[i + 1] = s.len() + size_sum[i];
    }
    for j in s {
        a[j] += size_sum[q] - size_sum[appear[j].unwrap()];
    }
    println!("{}", a.iter().join(" "));
}
