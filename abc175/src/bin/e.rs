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
    let mut dp = vec![vec![vec![[std::i64::MIN; 2]; 4]; c]; r];
    dp[0][0][0][0] = 0;
    for i in 0..r {
        for j in 0..c {
            for k in 0..4 {
                if field[i][j] != 0 && k < 3 {
                    dp[i][j][k + 1][1] = dp[i][j][k + 1][1].max(dp[i][j][k][0] + field[i][j]);
                }
                if j < c - 1 {
                    dp[i][j + 1][k][0] = dp[i][j + 1][k][0].max(dp[i][j][k][0]).max(dp[i][j][k][1]);
                }
                if i < r - 1 {
                    dp[i + 1][j][0][0] = dp[i + 1][j][0][0].max(dp[i][j][k][0]).max(dp[i][j][k][1]);
                }
            }
        }
    }
    let mut answer = std::i64::MIN;
    for k in 0..=3 {
        for flag in 0..2 {
            answer = answer.max(dp[r - 1][c - 1][k][flag]);
        }
    }
    println!("{}", answer);
}
