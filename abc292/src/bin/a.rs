use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    }
    println!("{}", s.iter().map(|c| (c - b'a' + b'A') as char).join(""));
}
