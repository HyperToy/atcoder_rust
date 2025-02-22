use proconio::*;

fn main() {
    input! {
        n: usize, w: i32, l: i32, r: i32,
        x: [i32; n],
    }
    let modulo = 1_000_000_000 + 7;
    let mut p = vec![0; n + 2];
    for i in 0..n {
        p[i + 1] = x[i];
    }
    p[n + 1] = w;

    let mut dp = vec![0; n + 2];
    let mut sum = vec![0; n + 2];
    dp[0] = 1;
    sum[0] = 1;
    for i in 1..=n + 1 {
        // refactor
        // p[i] - r <= p[x] を満たす最小の x を求める
        let mut ng: isize = -1;
        let mut ok: isize = n as isize + 1;
        while ng + 1 < ok {
            let wj = (ng + ok) / 2;
            if p[i] - r <= p[wj as usize] {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        let pos_l = ok;

        // p[x] <= p[i] - l  を満たす最大の x を求める
        let mut ok = -1;
        let mut ng = n as isize + 2;
        while ok + 1 < ng {
            let wj = (ok + ng) / 2;
            if p[wj as usize] <= p[i] - l {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        let pos_r = ok;

        dp[i] = (if pos_r >= 0 { sum[pos_r as usize] } else { 0 }
            - if pos_l > 0 {
                sum[pos_l as usize - 1]
            } else {
                0
            }
            + modulo)
            % modulo;
        sum[i] = (sum[i - 1] + dp[i]) % modulo;
    }

    println!("{}", dp[n + 1]);
}
