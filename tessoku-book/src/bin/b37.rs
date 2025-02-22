use proconio::*;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
    }

    let mut power10: Vec<usize> = vec![0; 16];
    power10[0] = 1;
    for i in 1..=15 {
        power10[i] = power10[i - 1] * 10;
    }

    let mut dp = vec![vec![0; 10]; 15];
    let mut answer = 0;
    for i in 0..=14 {
        // 10^i の位
        let digit = n % power10[i + 1] / power10[i];
        for j in 0..=9 {
            dp[i][j] = n / power10[i + 1] * power10[i]
                + match j.cmp(&(digit)) {
                    Ordering::Less => power10[i],
                    Ordering::Equal => n % power10[i] + 1,
                    Ordering::Greater => 0,
                };
            answer += j * dp[i][j];
        }
    }

    println!("{}", answer);
}
