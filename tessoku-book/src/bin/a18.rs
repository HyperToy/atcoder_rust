use proconio::*;

fn main() {
    input! {
        n: usize, s: usize,
        a: [usize; n],
    }
    // dp[i][j] := i 番目までで 合計を j にする方法はあるか
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            if dp[i][j] {
                dp[i + 1][j] = true;
                if j + a[i] <= s {
                    dp[i + 1][j + a[i]] = true;
                }
            }
        }
    }
    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
