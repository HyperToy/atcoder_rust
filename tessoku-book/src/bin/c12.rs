use proconio::*;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        es: [(usize, usize); m],
    }
    let mut dp = vec![vec![-1_000_000; n + 1]; k + 1];
    dp[0][0] = 0;
    for i in 1..=k {
        for j in 1..=n {
            for x in 0..j {
                dp[i][j] = dp[i][j].max(dp[i - 1][x] + score(x + 1, j, es.clone()));
            }
        }
    }
    println!("{}", dp[k][n]);
}
fn score(l: usize, r: usize, es: Vec<(usize, usize)>) -> i32 {
    let mut cnt = 0;
    for (a, b) in es {
        if l <= a && b <= r {
            cnt += 1;
        }
    }
    cnt
}
