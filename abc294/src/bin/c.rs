use itertools::Itertools;
use proconio::input;
use std::iter::repeat;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        b: [i32; m],
    }
    println!(
        "{}",
        a.iter()
            .zip(repeat('a'))
            .chain(b.iter().zip(repeat('b')))
            .sorted()
            .enumerate()
            .sorted_by_key(|&(_, (&x, c))| (c, x))
            .map(|(i, _)| i + 1)
            .join(" ")
    );
}
