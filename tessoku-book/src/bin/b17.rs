use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = dp[i].min(dp[i - 1] + (h[i] - h[i - 1]).abs());
        if i > 1 {
            dp[i] = dp[i].min(dp[i - 2] + (h[i] - h[i - 2]).abs());
        }
    }
    let mut answer = Vec::new();
    let mut place = n - 1;
    loop {
        answer.push(place);
        if place == 0 {
            break;
        }
        place -= if dp[place] == dp[place - 1] + (h[place] - h[place - 1]).abs() {
            1
        } else {
            2
        };
    }
    answer.reverse();
    println!("{}", answer.len());
    println!("{}", answer.iter().map(|x| x + 1).join(" "));
}
