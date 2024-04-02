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
    for x in xs {
        if s.contains(&x) {
            s.remove(&x);
        } else {
            s.insert(x);
        }
        for &j in &s {
            a[j] += s.len();
        }
    }
    todo!();
    println!("{}", a.iter().join(" "));
}
