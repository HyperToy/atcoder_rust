use proconio::*;

fn main() {
    input! {
        n: usize,
        mut problems: [(usize, usize); n],
    }
    problems.sort_by(|a, b| a.1.cmp(&b.1));
    let mut dp = vec![vec![0; 1441]; n + 1];
    for i in 0..n {
        let (t, d) = problems[i];
        for j in 0..=1440 {
            if t <= j && j <= d {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - t] + 1);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
