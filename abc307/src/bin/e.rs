use proconio::input;

const MOD: i64 = 998_244_353;

fn main() {
    input! {
        n: usize, m: i64,
    }
    let mut dp = vec![0; n.max(3)];
    dp[0] = m;
    dp[1] = m * (m - 1) % MOD;
    dp[2] = dp[1] * (m - 2) % MOD;
    for i in 3..n {
        dp[i] = ((m - 2) * dp[i - 1] % MOD + (m - 1) * dp[i - 2] % MOD) % MOD;
    }
    println!("{}", dp[n - 1]);
}
