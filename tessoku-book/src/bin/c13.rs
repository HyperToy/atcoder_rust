use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, p: u64,
        a: [u64; n],
    }
    const MODULO: u64 = 1_000_000_007;
    let mut map = HashMap::new();
    for x in a {
        *map.entry(x % MODULO).or_insert(0) += 1;
    }
    println!(
        "{}",
        if p == 0 {
            let c = if let Some(&x) = map.get(&0) { x } else { 0 };
            c * (c - 1) / 2 + c * (n - c)
        } else {
            let mut res = 0;
            for (x, value) in map.clone() {
                let y = p * pow(x, MODULO - 2, MODULO) % MODULO;
                res += if x == y {
                    value * (value - 1) / 2
                } else if x < y {
                    value * if let Some(&val) = map.get(&y) { val } else { 0 }
                } else {
                    0
                };
            }
            res
        }
    );
}

fn pow(mut a: u64, mut n: u64, modulo: u64) -> u64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 > 0 {
            res *= a;
            res %= modulo;
        }
        a *= a;
        a %= modulo;
        n /= 2;
    }
    res
}
