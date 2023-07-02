use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n - 1],
        b: [i32; n - 2],
    }
    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        if i >= 1 {
            dp[i] = dp[i].min(dp[i - 1] + a[i - 1]);
        }
        if i >= 2 {
            dp[i] = dp[i].min(dp[i - 2] + b[i - 2]);
        }
    }
    println!("{}", dp[n - 1]);
}
