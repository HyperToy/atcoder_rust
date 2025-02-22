use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, x: i32, y: i32,
        ab: [(i32, i32); n],
    }
    // dp[(i,j)] := i 個選んで 甘さの合計が j のときの、しょっぱさの合計の最小値
    let mut dp = HashMap::new();
    dp.insert((0, 0), 0);
    for (a, b) in ab {
        let mut next_dp: HashMap<(usize, i32), i32> = HashMap::new();
        for ((count, sweet), salty) in dp {
            // 使わない
            next_dp
                .entry((count, sweet))
                .and_modify(|e| *e = (*e).min(salty))
                .or_insert(salty);
            // 使う
            if sweet + a <= x {
                next_dp
                    .entry((count + 1, sweet + a))
                    .and_modify(|e| *e = (*e).min(salty + b))
                    .or_insert(salty + b);
            }
        }
        dp = next_dp;
    }
    println!(
        "{}",
        n.min(
            1 + dp
                .iter()
                .filter(|&(_, &k)| k <= y)
                .map(|((i, _), _)| i)
                .max()
                .unwrap()
        )
    );
}
