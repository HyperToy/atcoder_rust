use proconio::{marker::Chars, *};
// use itertools::Itertools;

fn main() {
    input! {
        _n: usize, k: usize,
        s: Chars,
    }
    let count = s.into_iter().filter(|x| x == &'1').count();
    println!("{}", if k % 2 == count % 2 { "Yes" } else { "No" });
}
