use ac_library::ModInt998244353 as Mint;
use proconio::{input, marker::Bytes};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut dp = HashMap::new();
    dp.insert((-1, -1), Mint::from(1));
    for i in 0..n {
        let mut next_dp = HashMap::new();
        for (&(a, b), &v) in &dp {
            for c in 0..26 {
                if s[i] != b'?' && b'a' + c as u8 != s[i] {
                    continue;
                }
                if c == a || c == b {
                    continue;
                }
                *(next_dp.entry((b, c)).or_insert(Mint::from(0))) += v;
            }
        }
        dp = next_dp;
    }
    println!("{}", dp.values().sum::<Mint>());
}
