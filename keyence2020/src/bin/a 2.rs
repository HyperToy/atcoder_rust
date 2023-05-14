use proconio::*;
use std::cmp::max;

fn main() {
    input! {
        h: u32,
        w: u32,
        n: u32,
    }
    let k = max(h, w);
    println!("{}", (n + k - 1) / k);
}
