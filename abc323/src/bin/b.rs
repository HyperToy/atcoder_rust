use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    println!(
        "{}",
        s.iter()
            .enumerate()
            .map(|(i, v)| (Reverse(v.iter().filter(|c| c == &&'o').count()), i))
            .sorted()
            .map(|(_win_count, i)| i + 1)
            .join(" ")
    );
}
