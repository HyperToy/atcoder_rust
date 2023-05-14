use proconio::*;
use std::cmp::min;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }
    println!("{}", min(a, b) + min(c, d));
}
