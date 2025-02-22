use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [Chars; h],
        b: [Chars; h],
    }
    println!(
        "{}",
        if (0..h).cartesian_product(0..w).any(|(s, t)| {
            (0..h)
                .cartesian_product(0..w)
                .all(|(i, j)| a[i][j] == b[(i + s) % h][(j + t) % w])
        }) {
            "Yes"
        } else {
            "No"
        }
    );
}
