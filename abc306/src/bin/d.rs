use proconio::input;

fn main() {
    input! {
        n: usize,
        dishes: [(i32, i64); n],
    }
    let y_min = dishes.iter().map(|(_, y)| *y).min().unwrap();

    // dp[i][j] := i 皿目までで、 j 状態のときの 美味しさの最大値
    // j: 0 (健康), 1 (お腹を壊している)
    let mut dp = vec![[std::i64::MIN - y_min.min(0); 2]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        let (x, y) = dishes[i];
        if x == 0 {
            // 解毒剤入り
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][0] + y).max(dp[i][1] + y);
        } else {
            // 毒入り
            dp[i + 1][1] = dp[i + 1][0].max(dp[i][0] + y);
        }
        dp[i + 1][0] = dp[i + 1][0].max(dp[i][0]);
        dp[i + 1][1] = dp[i + 1][1].max(dp[i][1]);
    }
    println!("{}", dp[n][0].max(dp[n][1]));
}
