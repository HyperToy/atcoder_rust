use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![[0; 2]; n];
    dp[0][1] = a[0];
    for i in 1..n {
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + 2 * a[i]);
        dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + a[i]);
    }
    println!("{}", dp[n - 1][0].max(dp[n - 1][1]));
}
