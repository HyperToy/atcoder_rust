use proconio::*;

fn main() {
    input! {
        w: u64
    }
    let modulo = 1_000_000_000 + 7;
    println!("{}", 12 * pow(7, w - 1, modulo) % modulo);
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
