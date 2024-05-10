use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        fs: [(Usize1, i32); n],
    }
    let mut set = HashSet::new();
    println!(
        "{}",
        fs.into_iter()
            .map(|(f, s)| (s, f))
            .sorted()
            .rev()
            .map(|(s, f)| {
                if set.contains(&f) {
                    s / 2
                } else {
                    set.insert(f);
                    s
                }
            })
            .sorted()
            .rev()
            .take(2)
            .sum::<i32>()
    );
}
