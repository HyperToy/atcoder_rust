use proconio::*;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }
    let mut dp = vec![vec![std::f64::MIN; n + 1]; n];
    dp[0][0] = 0.;
    dp[0][1] = p[0];
    for i in 1..n {
        for j in 0..=i + 1 {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            if j > 0 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] * 0.9 + p[i]);
            }
        }
    }
    let mut m = vec![0.; n + 1];
    m[0] = 0.;
    for i in 1..=n {
        m[i] = m[i - 1] * 0.9 + 1.;
    }
    let mut answer = std::f64::MIN;
    for j in 1..=n {
        answer = answer.max(dp[n - 1][j] / m[j] - 1200. / (j as f64).sqrt());
    }
    println!("{}", answer);
}
