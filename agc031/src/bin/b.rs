use ac_library::ModInt1000000007 as Mint;
use proconio::{input, marker::Usize1};

// with editorial
fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
    }
    let mut prev = vec![n; 200_000]; // prev[i] := 直前に色iが登場した位置
    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] += 1;
    for i in 0..n {
        let p = prev[c[i]];
        if p + 1 < i {
            dp[i] = dp[i] + dp[p];
        }
        dp[i + 1] = dp[i + 1] + dp[i];
        prev[c[i]] = i; // 登場した位置を更新
    }
    println!("{}", dp[n]);
}
