use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, k: usize,
        s: Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .sorted()
            .permutations(n)
            .sorted()
            .dedup()
            .filter(|s| s.windows(k).all(|s| !(0..k).all(|i| s[i] == s[k - 1 - i])))
            .count()
    );
}
