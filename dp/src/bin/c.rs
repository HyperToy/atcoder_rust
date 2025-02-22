use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [[i64; 3]; n],
    }
    let mut dp = vec![vec![std::i64::MIN; 3]; n];
    dp[0] = abc[0].clone();
    for i in 1..n {
        for j in 0..3 {
            for k in 0..3 {
                if j != k {
                    dp[i][j] = dp[i][j].max(dp[i - 1][k] + abc[i][j]);
                }
            }
        }
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
}
