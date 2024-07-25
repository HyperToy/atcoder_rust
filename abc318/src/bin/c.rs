use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, d: usize, p: i64,
        f: [i64; n],
    }
    println!(
        "{}",
        f.into_iter()
            .sorted()
            .rev()
            .chunks(d)
            .into_iter()
            .map(|v| p.min(v.sum()))
            .sum::<i64>()
    );
}
