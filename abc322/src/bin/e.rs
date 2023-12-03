use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, k: usize, p: i32,
        ca: [(i64, [i32; k]); n],
    }
    let mut dp = vec![HashMap::new(); n];
    dp[0].insert(ca[0].1.clone(), ca[0].0);
    dp[0].insert(vec![0; k], 0);
    for i in 1..n {
        let (now_cost, now_parameter) = &ca[i];
        for (parameter, cost) in dp[i - 1].clone() {
            // 使わない
            match dp[i].get(&parameter) {
                None => {
                    dp[i].insert(parameter.clone(), cost);
                }
                Some(x) if cost < *x => {
                    dp[i].insert(parameter.clone(), cost);
                }
                _ => (),
            }
            // 使う
            let new_parameter = (0..k)
                .map(|j| p.min(parameter[j] + now_parameter[j]))
                .collect::<Vec<_>>();
            match dp[i].get(&new_parameter) {
                None => {
                    dp[i].insert(new_parameter.clone(), cost + now_cost);
                }
                Some(x) if cost + now_cost < *x => {
                    dp[i].insert(new_parameter.clone(), cost + now_cost);
                }
                _ => (),
            }
        }
    }
    println!(
        "{}",
        match dp[n - 1].get(&vec![p; k]) {
            Some(x) => *x,
            None => -1,
        }
    )
}
