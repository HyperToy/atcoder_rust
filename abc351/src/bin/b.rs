use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    }
    println!(
        "{}",
        (0..n)
            .cartesian_product(0..n)
            .filter_map(|(i, j)| if a[i][j] == b[i][j] {
                None
            } else {
                Some(format!("{} {}", i + 1, j + 1))
            })
            .take(1)
            .join("")
    );
}
