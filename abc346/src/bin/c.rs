use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k:i64,
        a: [i64; n],
    }
    println!(
        "{}",
        k * (k + 1) / 2 - a.iter().filter(|x| **x <= k).sorted().dedup().sum::<i64>()
    );
}
