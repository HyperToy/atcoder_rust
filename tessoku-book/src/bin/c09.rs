use proconio::*;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 0;
    dp[0][1] = a[0];
    for i in 1..n {
        dp[i][0] = max(dp[i - 1][0], dp[i - 1][1]);
        dp[i][1] = dp[i - 1][0] + a[i];
    }
    println!("{}", max(dp[n - 1][0], dp[n - 1][1]));
}
