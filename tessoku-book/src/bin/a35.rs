use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut dp = vec![vec![-1; n]; n];
    for i in 0..n {
        dp[n - 1][i] = a[i];
    }
    for i in (0..(n - 1)).rev() {
        for j in 0..=i {
            if i % 2 == 0 {
                dp[i][j] = dp[i + 1][j].max(dp[i + 1][j + 1]);
            } else {
                dp[i][j] = dp[i + 1][j].min(dp[i + 1][j + 1]);
            }
        }
    }
    println!("{}", dp[0][0]);
}
