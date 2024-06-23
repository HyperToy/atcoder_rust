use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut t = Vec::new();
    for i in 0..s.len() / 2 {
        t.push(s[i * 2 + 1]);
        t.push(s[i * 2]);
    }
    println!("{}", t.into_iter().join(""));
}
