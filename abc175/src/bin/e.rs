use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (r, c): (usize, usize), k: usize,
        items: [(Usize1, Usize1, i64); k],
    }
    let mut field = vec![vec![0; c]; r];
    for (i, j, v) in items {
        assert_eq!(field[i][j], 0);
        field[i][j] = v;
    }
    // dp[i][j][k][flag] := マス(i, j) にいて、その行で k個のアイテムを拾っており、 flag (そのマスのアイテムを持っているか) のときの、
    // アイテムの価値の最大値
    let mut dp = vec![vec![[[std::i64::MIN; 2]; 4]; c]; r];
    dp[0][0][0][0] = 0;
    // 配る DP
    for i in 0..r {
        for j in 0..c {
            for k in 0..=3 {
                if field[i][j] > 0 && k < 3 {
                    // アイテムを拾う
                    dp[i][j][k + 1][1] = dp[i][j][k + 1][1].max(dp[i][j][k][0] + field[i][j]);
                }
                if j < c - 1 {
                    // 行の隣のマスに行く
                    dp[i][j + 1][k][0] = dp[i][j + 1][k][0].max(dp[i][j][k][0]).max(dp[i][j][k][1]);
                }
                if i < r - 1 {
                    // 隣の列に行く
                    dp[i + 1][j][0][0] = dp[i + 1][j][0][0].max(dp[i][j][k][0]).max(dp[i][j][k][1]);
                }
            }
        }
    }
    println!(
        "{}",
        dp[r - 1][c - 1]
            .iter()
            .map(|v| v.iter().max().unwrap())
            .max()
            .unwrap()
    );
}
