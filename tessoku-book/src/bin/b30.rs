use proconio::*;

fn main() {
    input! {
        h: u64, w: u64,
    }
    println!("{}", combination(h + w - 2, w - 1, 1_000_000_000 + 7));
}

fn combination(n: u64, r: u64, modulo: u64) -> u64 {
    let mut res = 1;
    for i in 0..r {
        res *= n - i;
        res %= modulo;
        res *= pow(i + 1, modulo - 2, modulo);
        res %= modulo;
    }
    res
}

fn pow(a: u64, n: u64, modulo: u64) -> u64 {
    let mut a = a;
    let mut n = n;
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
