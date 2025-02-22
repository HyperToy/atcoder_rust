use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut s = HashSet::new();
    for (a, b) in ab {
        s.insert((a + b) % n);
    }
    for k in 0..n {
        if s.len() == m {
            break;
        }
        s.insert(k);
    }
    println!(
        "{}\n{}",
        n * m,
        s.iter()
            .map(|k| (0..n).map(move |i| (i, (k + n - i) % n)))
            .flatten()
            .map(|(i, j)| format!("{} {}", i + 1, j + 1))
            .join("\n")
    );
}
