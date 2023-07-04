use proconio::*;

fn main() {
    input! {
        n: usize, w: usize,
        items: [(usize, usize); n],
    }
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 0..n {
        for j in 0..=w {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if j + items[i].0 <= w {
                dp[i + 1][j + items[i].0] = dp[i + 1][j + items[i].0].max(dp[i][j] + items[i].1);
            }
        }
    }
    println!("{}", dp[n][w]);
}
