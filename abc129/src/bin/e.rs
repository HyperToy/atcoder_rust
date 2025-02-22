use proconio::{input, marker::Chars};

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        l: Chars,
    }
    let l = l.into_iter().map(|c| c == '1').collect::<Vec<_>>();
    let n = l.len();
    let mut dp = vec![(0, 0); n + 1];
    dp[0].0 = 1;
    for i in 0..n {
        if l[i] {
            dp[i + 1].0 += dp[i].0 * 2;
            dp[i + 1].1 += dp[i].1 * 3 + dp[i].0;
        } else {
            dp[i + 1].0 += dp[i].0;
            dp[i + 1].1 += dp[i].1 * 3;
        }
        dp[i + 1].0 %= MOD;
        dp[i + 1].1 %= MOD;
    }
    println!("{}", (dp[n].0 + dp[n].1) % MOD);
}
