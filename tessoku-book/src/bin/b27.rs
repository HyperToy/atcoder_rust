use proconio::*;

fn main() {
    input! {
        a: u64, b: u64,
    }
    println!("{}", a * b / gcd(a, b));
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 || a == b {
        a
    } else if a > b {
        gcd(b, a % b)
    } else {
        gcd(a, b % a)
    }
}
