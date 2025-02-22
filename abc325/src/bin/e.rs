use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize, a: i64, b: i64, c: i64,
        dist: [[i64; n]; n],
    }
    let start = 0;
    let goal = n - 1;
    const CAR: usize = 0;
    const TRAIN: usize = 1;

    let mut dp = vec![vec![std::i64::MAX; n]; 2];
    let mut q = BinaryHeap::new();
    dp[CAR][start] = 0;
    q.push((Reverse(0), (CAR, start)));

    while let Some((Reverse(d), (t, u))) = q.pop() {
        if d > dp[t][u] {
            continue;
        }
        if t == CAR {
            if d <= dp[TRAIN][u] {
                dp[TRAIN][u] = d;
                q.push((Reverse(d), (TRAIN, u)));
            }
        }
        for v in 0..n {
            if v == u {
                continue;
            }
            let nd = d + match t {
                CAR => dist[u][v] * a,
                TRAIN => dist[u][v] * b + c,
                _ => unreachable!(),
            };
            if nd > dp[t][v] {
                continue;
            }
            dp[t][v] = nd;
            q.push((Reverse(nd), (t, v)));
        }
    }
    println!("{}", dp[TRAIN][goal])
}
