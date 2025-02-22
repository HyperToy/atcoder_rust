use proconio::*;

fn main() {
    input! {
        a: u64, b: u64,
    }
    println!("{}", pow(a, b, 1000000000 + 7));
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
