use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    println!("{}", s.iter().interleave(s.iter()).join(""));
}
