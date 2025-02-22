use proconio::*;

fn main() {
    input! {
        n: usize, w: usize,
        items: [(usize, usize); n], // w, v
    }
    let sum_v = items.iter().map(|(_, v)| v).sum();
    // dp[i][j] := i 番目までで 価値 j を達成するための重さの最小値
    let mut dp = vec![vec![None; sum_v + 1]; n + 1];
    dp[0][0] = Some(0);
    for i in 0..n {
        for j in 0..=sum_v {
            if let Some(x) = dp[i][j] {
                dp[i + 1][j] = Some(match dp[i + 1][j] {
                    Some(y) => y.min(x),
                    None => x,
                });
                if j + items[i].1 <= sum_v {
                    let x = x + items[i].0;
                    let j = j + items[i].1;
                    dp[i + 1][j] = Some(match dp[i + 1][j] {
                        Some(y) => y.min(x),
                        None => x,
                    });
                }
            }
        }
    }
    let mut answer = 0;
    for j in 0..=sum_v {
        if let Some(x) = dp[n][j] {
            if x <= w {
                answer = answer.max(j);
            }
        }
    }
    println!("{}", answer);
}
