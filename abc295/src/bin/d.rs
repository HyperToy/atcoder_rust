use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::convert::TryInto;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let s = s
        .into_iter()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect_vec();
    let mut count = vec![vec![false; 10]; n + 1];
    for i in 0..n {
        for j in 0..10 {
            count[i + 1][j] = (j == s[i]) ^ count[i][j];
        }
    }
    println!(
        "{}",
        count
            .into_iter()
            .counts()
            .into_iter()
            .map(|(_, v)| v * (v - 1) / 2)
            .sum::<usize>()
    );
}
