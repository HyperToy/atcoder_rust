use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        s: Chars
    }
    let t = ['M', 'E', 'X'];
    let l = t.len();
    let mut dp = vec![vec![0; 1 << l]; l + 1];
    dp[0][0] = 1;
    for (&a, &s) in a.iter().zip(s.iter()) {
        let mut next_dp = dp.clone();
        for (j, &t) in t.iter().enumerate() {
            for mask in 0..(1 << l) {
                if s == t {
                    next_dp[j + 1][mask | 1 << a] += dp[j][mask];
                }
            }
        }
        dp = next_dp;
    }
    println!(
        "{}",
        dp[l]
            .iter()
            .enumerate()
            .map(|(mask, count)| count * (0..).find(|&i| (mask >> i) & 1 == 0).unwrap())
            .sum::<i64>()
    );
    /* 別解
    use std::collections::HashMap;
    let mut dp = HashMap::new();
    dp.insert((0, 0), 1);
    for (&a, &s) in a.iter().zip(s.iter()) {
        let mut next_dp = HashMap::new();
        for (&(j, mask), &count) in &dp {
            *(next_dp.entry((j, mask)).or_insert(0)) += count;
            if j < l && s == t[j] {
                *(next_dp.entry((j + 1, mask | 1 << a)).or_insert(0)) += count;
            }
        }
        dp = next_dp;
    }
    println!(
        "{}",
        dp.iter()
            .filter(|(&(j, _), _)| j == 3)
            .map(|((_, mask), count)| count * (0..).find(|&i| (mask >> i) & 1 == 0).unwrap())
            .sum::<i64>()
    );
    */
}
