use proconio::*;

fn main() {
    input! {
        n: usize, a: usize, b: usize,
    }
    let mut dp = vec![false; n + 1];
    for i in 0..=n {
        // 負け状態への遷移が可能なら勝ち状態
        if i >= a {
            dp[i] |= !dp[i - a];
        }
        if i >= b {
            dp[i] |= !dp[i - b];
        }
    }
    println!("{}", if dp[n] { "First" } else { "Second" });
}
