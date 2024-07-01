use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(usize, usize, usize); n],
    }
    let costs = xyz
        .iter()
        .map(|&(x, y, _)| if x > y { 0 } else { (y - x + 1) / 2 })
        .collect_vec();
    let values = xyz.iter().map(|&(_, _, z)| z).collect_vec();
    let total = values.iter().sum::<usize>();
    // costs[i] で values[i] が得られる
    // dp[i][j] := 選挙区i までで j議席取るのに必要な最小コスト
    let mut dp = vec![vec![std::usize::MAX / 2; total + 1]; n];
    dp[0][0] = 0;
    dp[0][values[0]] = costs[0];
    for i in 1..n {
        for j in 0..=total {
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            if j >= values[i] {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - values[i]] + costs[i]);
            }
        }
    }
    println!("{}", dp[n - 1].iter().skip((total + 1) / 2).min().unwrap());
}
