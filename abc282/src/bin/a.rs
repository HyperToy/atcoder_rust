use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    println!("{}", ('A'..).take(k).join(""));
}
