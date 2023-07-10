use proconio::*;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; k],
    }
    let mut dp = vec![false; n + 1];
    for i in 0..=n {
        for j in 0..k {
            if i >= a[j] {
                dp[i] |= !dp[i - a[j]];
            }
        }
    }
    println!("{}", if dp[n] { "First" } else { "Second" });
}
