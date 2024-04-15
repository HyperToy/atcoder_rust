use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    let l = s.len();
    s[l - 1] = '4';
    println!("{}", s.iter().join(""));
}
