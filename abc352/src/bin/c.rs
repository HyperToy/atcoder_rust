use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let ab = ab
        .into_iter()
        .sorted_by(|&(a1, b1), &(a2, b2)| (b1 - a1).cmp(&(b2 - a2)))
        .collect_vec();
    println!(
        "{}",
        ab.iter().map(|(a, _)| a).sum::<i64>() + ab.iter().map(|(a, b)| b - a).last().unwrap()
    );
}
