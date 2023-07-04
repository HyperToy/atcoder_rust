use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize, s: usize,
        a: [usize; n],
    }
    // dp[i][j] := i 番目までで 合計を j にする方法はあるか
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            if dp[i][j] {
                dp[i + 1][j] = true;
                if j + a[i] <= s {
                    dp[i + 1][j + a[i]] = true;
                }
            }
        }
    }
    if !dp[n][s] {
        println!("{}", -1);
        return;
    }
    let mut answer = Vec::new();
    let mut number = s;
    for i in (0..n).rev() {
        if number >= a[i] && dp[i][number - a[i]] {
            answer.push(i);
            number -= a[i];
        }
    }
    answer.reverse();
    println!("{}", answer.len());
    println!("{}", answer.iter().map(|x| x + 1).join(" "));
}
