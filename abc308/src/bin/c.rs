use itertools::Itertools;
use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    println!(
        "{}",
        ab.iter()
            .zip(0..n)
            .sorted_by(|((a1, b1), i), ((a2, b2), j)| {
                let c = (a1 * (a2 + b2)).cmp(&(a2 * (a1 + b1)));
                if c == Ordering::Equal {
                    j.cmp(&i)
                } else {
                    c
                }
            })
            .rev()
            .map(|((_a, _b), i)| i + 1)
            .join(" ")
    );
}
