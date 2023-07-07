use proconio::*;

fn main() {
    input! {
        n: usize, m: usize, // n ≤ 10, m ≤ 100
        a: [[u32; n]; m],
    }
    // dp[i][S] := i 番目のクーポンで、 Sの商品をすべて買うために必要な、クーポンの最小枚数
    let mut dp = vec![vec![10000; 1 << n]; m + 1];
    dp[0][0] = 0;
    for i in 0..m {
        for mask in 0..(1 << n) {
            dp[i + 1][mask] = dp[i + 1][mask].min(dp[i][mask]);
            let mut now = 0;
            for j in 0..n {
                if a[i][j] == 1 {
                    now |= 1 << j;
                }
            }
            dp[i + 1][mask | now] = dp[i + 1][mask | now].min(dp[i][mask] + 1);
        }
    }
    println!(
        "{}",
        if dp[m][(1 << n) - 1] < 10000 {
            dp[m][(1 << n) - 1]
        } else {
            -1
        }
    );
}
