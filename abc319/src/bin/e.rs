use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, x: usize, y: usize,
        pt: [(usize, usize); n - 1],
        q: usize,
        queries: [usize; q],
    }
    let cycle = 840; // 1..=8 の最小公倍数
    let time = (0..cycle)
        .map(|a| a + x)
        .map(|a| pt.iter().fold(a, |a, &(p, t)| (a + p - 1) / p * p + t))
        .map(|a| a + y)
        .collect_vec();
    println!(
        "{}",
        queries
            .into_iter()
            .map(|t| t / cycle * cycle + time[t % cycle])
            .join("\n")
    );
}
