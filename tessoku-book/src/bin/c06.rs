use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", n);
    println!(
        "{}",
        (0..n)
            .map(|x| format!("{} {}", x + 1, (x + 1) % n + 1))
            .join(" ")
    );
}
