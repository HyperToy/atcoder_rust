use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        p: [Usize1; n-1],
    }
    let mut depth = vec![0; n];
    for i in 0..n - 1 {
        depth[i + 1] = depth[p[i]] + 1;
    }
    let mut b = vec![0; depth.iter().max().unwrap() + 1];
    for (&a, &d) in a.iter().zip(depth.iter()) {
        b[d] += a;
    }
    println!(
        "{}",
        match b.into_iter().rev().find_or_last(|&v| v != 0).unwrap() {
            x if x > 0 => "+",
            x if x < 0 => "-",
            _ => "0",
        }
    )
}
