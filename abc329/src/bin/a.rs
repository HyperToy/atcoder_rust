use itertools::Itertools;
use proconio::{marker::Chars, *};

fn main() {
    input! {
        s:Chars,
    }
    println!("{}", s.iter().join(" "));
}
