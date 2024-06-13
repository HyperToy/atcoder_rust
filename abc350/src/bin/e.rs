use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64, a: u64, x: f64, y: f64,
    }
    let mut dp = HashMap::new();
    println!("{}", rec(n, a, x, y, &mut dp));
}

fn rec(n: u64, a: u64, x: f64, y: f64, dp: &mut HashMap<u64, f64>) -> f64 {
    if let Some(&res) = dp.get(&n) {
        return res;
    }
    if n == 0 {
        return 0.;
    }
    let res_a = x + rec(n / a, a, x, y, dp);
    let mut res_b = y;
    for b in 2..=6 {
        res_b += y + rec(n / b, a, x, y, dp);
    }
    res_b /= 5.;
    let res = res_a.min(res_b);
    dp.insert(n, res);
    res
}
