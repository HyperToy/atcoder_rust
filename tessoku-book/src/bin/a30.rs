use proconio::*;

fn main() {
    input! {
        n: u64, r: u64,
    }
    println!("{}", combination(n, r, 1000000000 + 7));
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
