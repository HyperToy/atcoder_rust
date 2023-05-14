use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, x: i32, y: i32,
        a: [i32; n],
    }
    let mut dp = vec![HashSet::new(); 2];
    dp[0].insert(a[0]);
    dp[1].insert(0);
    for i in 1..n {
        let mut next_dp = HashSet::new();
        for y in &dp[i % 2] {
            next_dp.insert(y + a[i]);
            next_dp.insert(y - a[i]);
        }
        dp[i % 2] = next_dp;
    }
    println!(
        "{}",
        if dp[0].contains(&x) && dp[1].contains(&y) {
            "Yes"
        } else {
            "No"
        }
    )
}
