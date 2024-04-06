use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        h: [i64; n],
    }
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        for j in i + 1..n.min(i + k + 1) {
            dp[j] = dp[j].min(dp[i] + (h[i] - h[j]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}
