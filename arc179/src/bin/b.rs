use ac_library::ModInt998244353 as Mint;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        m: usize, n: usize,
        x: [Usize1; m],
    }
    let mut x_inverse = vec![Vec::new(); m];
    for i in 0..m {
        x_inverse[x[i]].push(i);
    }
    let mut dp = vec![vec![vec![Mint::from(0); 1 << m]; m]; n];
    let all = (1usize << m) - 1;
    for j in 0..m {
        let mask = all ^ if x[j] == j { 0 } else { 1 << j };
        dp[0][j][mask] += 1;
    }
    for i in 0..n - 1 {
        for j in 0..m {
            for k in 0..m {
                for mask in 0..(1 << m) {
                    if mask >> k & 1 == 0 {
                        continue;
                    }
                    let mut next_mask = mask;
                    next_mask &= all ^ (1 << k); // k を使うので kを下ろす
                    for &x in &x_inverse[k] {
                        next_mask |= 1 << x;
                    }
                    dp[i + 1][k][next_mask] = dp[i + 1][k][next_mask] + dp[i][j][mask];
                }
            }
        }
    }
    println!("{}", dp[n - 1].iter().flatten().sum::<Mint>());
}
