use proconio::{marker::Usize1, *};
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        pa: [(Usize1, i32); n],
    }

    // hack refactor
    let mut p = Vec::new();
    let mut a = Vec::new();
    for i in 0..n {
        p.push(pa[i].0);
        a.push(pa[i].1);
    }

    // dp[l][r] := [l, r] の状態になるまでの得点の最大値
    let mut dp = vec![vec![0; n]; n];
    dp[0][n - 1] = 0;

    // 配る dp
    for l in 0..n {
        for r in (0..n).rev() {
            if l >= r {
                break;
            }
            dp[l + 1][r] = max(
                dp[l + 1][r],
                dp[l][r] + if l < p[l] && p[l] <= r { a[l] } else { 0 },
            );
            dp[l][r - 1] = max(
                dp[l][r - 1],
                dp[l][r] + if l <= p[r] && p[r] < r { a[r] } else { 0 },
            );
        }
    }

    // hack refactor
    let mut answer = 0;
    for i in 0..n {
        answer = max(answer, dp[i][i]);
    }
    println!("{}", answer);
}
