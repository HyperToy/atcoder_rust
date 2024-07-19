use ac_library::ModInt998244353 as Mint;
use proconio::input;

const M: usize = 26;
fn main() {
    input! {
        k: usize,
        c: [usize; M],
    }
    let mut comb = vec![vec![Mint::from(0); k + 1]; k + 1];
    for n in 0..=k {
        for r in 0..=n {
            comb[n][r] = if n == r || r == 0 {
                Mint::from(1)
            } else {
                comb[n - 1][r - 1] + comb[n - 1][r]
            };
        }
    }
    let mut dp = vec![vec![Mint::from(0); k + 1]; M + 1];
    dp[0][0] = Mint::from(1);
    for i in 1..=M {
        for j in 0..=k {
            let mut now = Mint::from(0);
            for t in 0..=c[i - 1].min(j) {
                now += dp[i - 1][j - t] * comb[j][t];
            }
            dp[i][j] = now;
        }
    }
    println!("{}", dp[M].iter().skip(1).sum::<Mint>());
}
