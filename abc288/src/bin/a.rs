use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n],
    }
    println!("{}", ab.iter().map(|&(a, b)| a + b).join("\n"));
}
