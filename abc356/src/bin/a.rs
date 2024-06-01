use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, l: usize, r: usize,
    }
    println!(
        "{}",
        (0..n)
            .map(|a| a + 1)
            .map(|a| if a < l || r < a { a } else { l + r - a })
            .join(" ")
    )
}
