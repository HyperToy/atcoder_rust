use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let s = s
        .iter()
        .map(|c| if *c == '0' { 0 } else { 1 })
        .collect::<Vec<_>>();
    // dp[i].k := i 番目までみたときの、 k (=0,1) の個数
    let mut dp = vec![(0, 0); n];
    if s[0] == 0 {
        dp[0].0 = 1;
    } else {
        dp[0].1 = 1;
    }
    for i in 1..n {
        if s[i] == 0 {
            dp[i].1 = dp[i - 1].0 + dp[i - 1].1;
            dp[i].0 = 1;
        } else {
            dp[i].0 = dp[i - 1].1;
            dp[i].1 = dp[i - 1].0 + 1;
        }
    }
    let answer = dp.iter().map(|(_, x)| x).sum::<i64>();
    println!("{}", answer);
}
