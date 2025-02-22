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
    let mut dp = vec![vec![None; n + 1]; n];
    for i in 0..n {
        dp[i][i] = Some(0);
    }
    for len in 2..=n {
        for l in 0..n {
            let r = l + len;
            if r > n {
                break;
            }
            let mut now = if let Some(x) = dp[l + 1][r - 1] {
                Some(x + (a[l] - a[r - 1]).abs())
            } else {
                None
            };
            for m in l..r {
                if let Some(x) = dp[l][m] {
                    if let Some(y) = dp[m][r] {
                        now = if let Some(v) = now {
                            Some(v.min(x + y))
                        } else {
                            Some(x + y)
                        }
                    }
                }
            }
            dp[l][r] = if let Some(x) = dp[l][r] {
                if let Some(v) = now {
                    Some(x.min(v))
                } else {
                    Some(x)
                }
            } else {
                now
            }
        }
    }
    if let Some(x) = dp[0][n] {
        x
    } else {
        -1
    }
}
