use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    println!("{}", (0..n).map(|_| n.to_string()).join(""));
}
