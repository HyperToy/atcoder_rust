use proconio::{marker::Chars, *};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j + 1]);
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i + 1][j]);
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
            }
        }
    }
    println!("{}", dp[n][m]);
}

trait ChangeMinMax {
    fn change_min(&mut self, v: Self) -> bool;
    fn change_max(&mut self, v: Self) -> bool;
}
impl<T: PartialOrd> ChangeMinMax for T {
    fn change_min(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn change_max(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}
