use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!(
        "{}",
        (0..n)
            .map(|i| if (i + 1) % 3 == 0 { "x" } else { "o" })
            .join("")
    );
}
