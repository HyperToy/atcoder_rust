use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    println!(
        "{}",
        (1..n)
            .map(|i| (0..n - i).take_while(|&k| s[k] != s[k + i]).count())
            .join("\n")
    );
}
