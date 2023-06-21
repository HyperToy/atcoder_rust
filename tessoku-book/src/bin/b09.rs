use proconio::*;

fn main() {
    input! {
        n: usize,
        ts: [((usize, usize), (usize, usize)); n],
    }
    let mut dp = vec![vec![0; 1501]; 1501];
    for ((a, b), (c, d)) in ts {
        dp[a][b] += 1;
        dp[a][d] -= 1;
        dp[c][b] -= 1;
        dp[c][d] += 1;
    }
    for i in 0..=1500 {
        for j in 1..=1500 {
            dp[i][j] += dp[i][j - 1];
        }
    }
    for j in 0..=1500 {
        for i in 1..=1500 {
            dp[i][j] += dp[i - 1][j];
        }
    }
    let mut answer = 0;
    for i in 0..=1500 {
        for j in 0..=1500 {
            answer += if dp[i][j] > 0 { 1 } else { 0 };
        }
    }
    println!("{}", answer);
}
