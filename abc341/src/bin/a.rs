use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", (0..n).map(|_| "10").join("") + "1");
}
