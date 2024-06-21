use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    }
    let n = n.into_iter().map(|c| (c - b'0') as usize).collect_vec();
    let mut answer = 0u64;
    for s in 1..=9 * 14 {
        let mut dp = vec![vec![vec![vec![0; 2]; s]; s + 1]; n.len() + 1];
        dp[0][0][0][1] = 1;
        for d in 0..n.len() {
            for i in 0..=s {
                for j in 0..s {
                    for f in 0..2 {
                        for t in 0..10 {
                            if i + t > s {
                                continue;
                            }
                            if f == 1 && n[d] < t {
                                continue;
                            }
                            dp[d + 1][i + t][(j * 10 + t) % s]
                                [if f == 1 && n[d] == t { 1 } else { 0 }] += dp[d][i][j][f];
                        }
                    }
                }
            }
        }
        answer += dp[n.len()][s][0][0] + dp[n.len()][s][0][1];
    }
    println!("{}", answer);
}
