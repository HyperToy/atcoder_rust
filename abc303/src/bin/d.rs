use proconio::marker::Chars;
use proconio::*;
use std::cmp::min;

fn main() {
    input! {
        x: u64, y: u64, z: u64,
        s: Chars,
    }
    let n = s.len();
    let max: u64 = 1_000_000_000_000_000_000;
    let mut dp = vec![vec![max; 2]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        dp[i][0] = min(dp[i][0], dp[i][1] + z);
        dp[i][1] = min(dp[i][1], dp[i][0] + z);
        match s[i] {
            'a' => {
                dp[i + 1][0] = min(dp[i + 1][0], dp[i][0] + x);
                dp[i + 1][1] = min(dp[i + 1][1], dp[i][1] + y);
            }
            'A' => {
                dp[i + 1][0] = min(dp[i + 1][0], dp[i][0] + y);
                dp[i + 1][1] = min(dp[i + 1][1], dp[i][1] + x);
            }
            _ => (),
        }
    }
    println!("{}", dp[n][0].min(dp[n][1]));
}
