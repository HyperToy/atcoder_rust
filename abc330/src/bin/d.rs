use itertools::Itertools;
use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut is = vec![0; n];
    let mut js = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                is[i] += 1;
                js[j] += 1;
            }
        }
    }
    println!(
        "{}",
        (0..n)
            .cartesian_product(0..n)
            .filter(|(i, j)| s[*i][*j] == 'o')
            .map(|(i, j)| (is[i] - 1) * (js[j] - 1))
            .sum::<u64>()
    );
}
