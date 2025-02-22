use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
        queries: [(usize, usize); q],
    }
    let mut culminate = vec![0; n + 1];
    for i in 1..n {
        culminate[i + 1] = culminate[i];
        if s[i] == s[i - 1] {
            culminate[i + 1] += 1;
        }
    }
    println!(
        "{}",
        queries
            .into_iter()
            .map(|(l, r)| culminate[r] - culminate[l])
            .join(" ")
    );
}
