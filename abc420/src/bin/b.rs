use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    }
    let winners = (0..m)
        .map(|j| {
            let x = (0..n).filter(|i| s[*i][j] == '0').count();
            let y = (0..n).filter(|i| s[*i][j] == '1').count();
            match (x, y) {
                (0, _) => '1',
                (_, 0) => '0',
                (x, y) if x < y => '0',
                (x, y) if x > y => '1',
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();
    let scores = s
        .iter()
        .map(|v| v.iter().zip(&winners).filter(|(v, w)| v == w).count())
        .collect::<Vec<_>>();
    let max = scores.iter().max().unwrap();
    println!(
        "{}",
        scores
            .iter()
            .enumerate()
            .filter_map(|(i, s)| if s == max { Some(i + 1) } else { None })
            .join(" ")
    );
}
