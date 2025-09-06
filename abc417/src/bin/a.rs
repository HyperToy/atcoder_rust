use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, a: usize, b: usize,
        s: Chars,
    }
    println!("{}", &s[a..n - b].iter().join(""));
}
