use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        w: [u64; n],
    }
    let mut v = vec![Vec::new(); n];
    for i in 0..n {
        v[a[i]].push(w[i]);
    }
    println!(
        "{}",
        v.into_iter()
            .map(|v| v.into_iter().sorted().rev().skip(1).sum::<u64>())
            .sum::<u64>()
    );
}
