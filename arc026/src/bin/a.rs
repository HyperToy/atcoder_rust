use proconio::*;
use std::cmp::*;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }
    println!("{}", a * max(n - 5, 0) + b * min(n, 5));
}
