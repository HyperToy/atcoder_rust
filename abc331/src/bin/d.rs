use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, q: usize,
        p: [Chars; n],
        queries: [(usize, usize, usize, usize); q],
    }
    let mut s = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            s[i][j] = if p[i][j] == 'B' { 1 } else { 0 }
                + if i > 0 { s[i - 1][j] } else { 0 }
                + if j > 0 { s[i][j - 1] } else { 0 }
                - if i > 0 && j > 0 { s[i - 1][j - 1] } else { 0 };
        }
    }
    println!(
        "{}",
        queries
            .into_iter()
            .map(|(a, b, c, d)| {
                calc(n, &s, c, d)
                    + if a > 0 && b > 0 {
                        calc(n, &s, a - 1, b - 1)
                    } else {
                        0
                    }
                    - if a > 0 { calc(n, &s, a - 1, d) } else { 0 }
                    - if b > 0 { calc(n, &s, c, b - 1) } else { 0 }
            })
            .join("\n")
    );
}

fn calc(n: usize, s: &Vec<Vec<usize>>, a: usize, b: usize) -> usize {
    s[n - 1][n - 1] * (a / n) * (b / n)
        + s[a % n][n - 1] * (b / n)
        + s[n - 1][b % n] * (a / n)
        + s[a % n][b % n]
}
