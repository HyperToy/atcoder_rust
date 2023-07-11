use proconio::*;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
    }
    let mut power10: Vec<usize> = Vec::new();
    power10.push(1);
    for i in 1..=15 {
        power10.push(power10[i - 1] * 10);
    }
    let mut dp = vec![vec![0; 10]; 15];
    for i in 0..=14 {
        for j in 0..=9 {
            dp[i][j] = n / power10[i + 1] * power10[i]
                + match j.cmp(&(n % power10[i + 1] / power10[i])) {
                    Ordering::Less => power10[i],
                    Ordering::Equal => n % power10[i] + 1,
                    Ordering::Greater => 0,
                }
        }
    }

    let mut answer = 0;
    for i in 0..=14 {
        for j in 0..=9 {
            answer += j * dp[i][j];
        }
    }
    println!("{}", answer);
}
