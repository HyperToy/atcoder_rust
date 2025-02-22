use ac_library::ModInt998244353 as Mint;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64
    }
    let mut dp = HashMap::new();
    println!("{}", dfs(1, n, &mut dp));
}

// メモ化再帰
fn dfs(n: u64, target: u64, dp: &mut HashMap<u64, Mint>) -> Mint {
    if let Some(&x) = dp.get(&n) {
        return x;
    }
    let res = if n > target {
        Mint::from(0)
    } else if target % n != 0 {
        Mint::from(0)
    } else if n == target {
        Mint::from(1)
    } else {
        (2..=6).map(|i| dfs(n * i, target, dp)).sum::<Mint>() / 5 // 確率漸化式
    };
    dp.insert(n, res);
    res
}
