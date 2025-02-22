use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
        b: [Usize1; n - 1],
    }
    let mut dp = vec![std::i32::MIN; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[a[i]] = dp[a[i]].max(dp[i] + 100);
        dp[b[i]] = dp[b[i]].max(dp[i] + 150);
    }
    println!("{}", dp[n - 1]);
}
