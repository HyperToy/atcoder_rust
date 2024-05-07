use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        (h, w): (usize, usize), n: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![0; w + 1]; h + 1];
    let holes = ab.into_iter().collect::<HashSet<_>>();
    for i in 1..=h {
        for j in 1..=w {
            if !holes.contains(&(i, j)) {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }
    println!("{}", dp.iter().flatten().sum::<u64>());
}
