use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut called = HashSet::new();
    for (i, &a) in a.iter().enumerate() {
        if !called.contains(&i) {
            called.insert(a);
        }
    }
    println!(
        "{}\n{}",
        n - called.len(),
        (0..n)
            .filter(|&i| !called.contains(&i))
            .map(|i| i + 1)
            .join(" ")
    );
}
