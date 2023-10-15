use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n * 2],
    }
    println!("{}", solve(a));
}

fn solve(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut dp = vec![vec![1_000_000_000; n + 1]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for len in 2..=n {
        for l in 0..n {
            let r = l + len;
            if r > n {
                break;
            }
            dp[l][r] = dp[l][r].min(dp[l + 1][r - 1] + (a[l] - a[r - 1]).abs());
            for m in l..r {
                dp[l][r] = dp[l][r].min(dp[l][m] + dp[m][r]);
            }
        }
    }
    dp[0][n]
}
