use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=2 {
            if i + j < n {
                dp[i + j] = dp[i + j].min(dp[i] + (h[i] - h[i + j]).abs());
            }
        }
    }
    println!("{}", dp[n - 1]);
}
