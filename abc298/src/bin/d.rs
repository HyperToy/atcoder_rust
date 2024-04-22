use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;
const MOD: u64 = 998244353;

fn main() {
    input! {
        q: usize,
    }
    let mut n = 1;
    let mut xs = VecDeque::new();
    xs.push_back(1);
    let mut k = 0;
    let mut answers = Vec::new();
    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! { x: u64 }
                n = (n * 10 + x) % MOD;
                xs.push_back(x);
                k += 1;
            }
            2 => {
                let x = xs.pop_front().unwrap();
                let x = (x * pow(10, k)) % MOD;
                n = (n - x + MOD) % MOD;
                k -= 1;
            }
            3 => {
                answers.push(n);
            }
            _ => unreachable!(),
        }
    }
    println!("{}", answers.iter().join("\n"));
}

fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 > 0 {
            res *= a;
            res %= MOD;
        }
        a *= a;
        a %= MOD;
        n /= 2;
    }
    res
}
