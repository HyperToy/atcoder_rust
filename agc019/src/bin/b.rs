use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
    }
    let comb = |x| x * (x - 1) / 2;
    println!(
        "{}",
        comb(a.len()) + 1
            - a.into_iter()
                .counts()
                .into_iter()
                .map(|(_, v)| comb(v))
                .sum::<usize>()
    );
}
