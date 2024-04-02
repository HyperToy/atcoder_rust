use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, (a, b): (i64, i64),
        d: [i64; n],
    }
    let d = d.iter().map(|x| x % (a + b)).collect::<Vec<_>>();
    let d = d
        .clone()
        .iter()
        .map(|&x| x + (a + b))
        .chain(d.into_iter())
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    let mut longest_gap = 0;
    for i in 1..d.len() {
        longest_gap = longest_gap.max(d[i] - d[i - 1] - 1);
    }
    println!("{}", if longest_gap >= b { "Yes" } else { "No" });
}
