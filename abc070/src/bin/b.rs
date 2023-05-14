use proconio::*;
use std::cmp::{min, max};

fn main() {
    input! {
        a: i32, b: i32,
        c: i32, d: i32,
    }
    println!("{}", max(0, min(b, d) - max(a, c)));
}
