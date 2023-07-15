use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, q: usize,
        a: [Usize1; n],
        qs: [(Usize1, usize); q],
    }
    // dp[i][j] := 位置 i にいた 2^j 日後にいる位置
    let mut dp = vec![vec![0; 30]; n];
    for i in 0..n {
        dp[i][0] = a[i];
    }
    for j in 1..30 {
        for i in 0..n {
            dp[i][j] = dp[dp[i][j - 1]][j - 1];
        }
    }
    for (x, y) in qs {
        let mut pos = x;
        for i in 0..30 {
            if (y >> i & 1) == 1 {
                pos = dp[pos][i];
            }
        }
        println!("{}", pos + 1);
    }
}
