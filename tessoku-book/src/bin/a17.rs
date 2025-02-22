use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n - 1],
        b: [i32; n - 2],
    }
    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        if i >= 1 {
            dp[i] = dp[i].min(dp[i - 1] + a[i - 1]);
        }
        if i >= 2 {
            dp[i] = dp[i].min(dp[i - 2] + b[i - 2]);
        }
    }
    let mut answer = Vec::new();
    let mut place = n - 1;
    loop {
        answer.push(place);
        if place == 0 {
            break;
        }
        place -= if dp[place] == dp[place - 1] + a[place - 1] {
            1
        } else {
            2
        };
    }
    answer.reverse();
    println!("{}", answer.len());
    println!("{}", answer.iter().map(|x| x + 1).join(" "));
}
