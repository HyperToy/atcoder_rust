use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i64; n * (n - 1) / 2],
    }
    let mut g = vec![vec![0; n]; n];
    for (d, (i, j)) in d
        .into_iter()
        .zip((0..n).map(|i| (i + 1..n).map(move |j| (i, j))).flatten())
    {
        g[i][j] = d;
    }
    let mut dp = vec![0; 1 << n];
    for b in 0..(1 << n) {
        if let Some(u) = (0..n).find(|&i| b >> i & 1 == 0) {
            for v in 0..n {
                if b >> v & 1 == 0 {
                    let nb = b | (1 << u) | (1 << v);
                    dp[nb] = dp[nb].max(dp[b] + g[u][v]);
                }
            }
        }
    }
    println!("{}", dp[(1 << n) - 1])
}
