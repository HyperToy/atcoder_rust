use proconio::*;

fn main() {
    input! {
        n: usize, k: usize,
    }
    // dp[i][j] := i に 2^j 回操作を行ったときの値
    let mut dp = vec![vec![0; 40]; n + 1];
    for i in 0..=n {
        dp[i][0] = operate(i);
    }
    for j in 1..40 {
        for i in 0..=n {
            dp[i][j] = dp[dp[i][j - 1]][j - 1];
        }
    }

    for i in 1..=n {
        let mut result = i;
        for j in 0..40 {
            if (k >> j & 1) == 1 {
                result = dp[result][j];
            }
        }
        println!("{}", result);
    }
}

fn operate(mut x: usize) -> usize {
    let mut result = x;
    while x > 0 {
        result -= x % 10;
        x /= 10;
    }
    result
}
