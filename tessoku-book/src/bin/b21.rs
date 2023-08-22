use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for r in 0..n {
        for l in (0..r).rev() {
            dp[l][r] = if s[l] == s[r] {
                2 + dp[l + 1][r - 1]
            } else {
                std::cmp::max(dp[l][r - 1], dp[l + 1][r])
            };
        }
    }
    println!("{}", dp[0][n - 1])
}
