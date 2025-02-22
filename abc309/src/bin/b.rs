use itertools::Itertools;
use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut b = vec![vec!['0'; n]; n];
    for i in 0..n {
        for j in 0..n {
            if 0 < i && i < n - 1 && 0 < j && j < n - 1 {
                b[i][j] = a[i][j];
            } else if i == 0 && j < n - 1 {
                b[i][j + 1] = a[i][j];
            } else if i == n - 1 && j > 0 {
                b[i][j - 1] = a[i][j];
            } else if j == 0 && i > 0 {
                b[i - 1][j] = a[i][j];
            } else if j == n - 1 && i < n - 1 {
                b[i + 1][j] = a[i][j];
            } else {
                unreachable!();
            }
        }
    }
    for i in 0..n {
        println!("{}", b[i].iter().join(""));
    }
}
