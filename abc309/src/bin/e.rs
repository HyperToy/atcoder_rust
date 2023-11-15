use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        p: [Usize1; n - 1],
        xy: [(Usize1, i32); m],
    }
    let mut dp = vec![-1; n];
    for (x, y) in xy {
        dp[x] = dp[x].max(y);
    }
    for i in 1..n {
        dp[i] = dp[i].max(dp[p[i - 1]] - 1)
    }
    println!("{}", dp.iter().filter(|x| **x >= 0).count());
}
