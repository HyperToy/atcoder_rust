use proconio::*;

fn main() {
    input! {
        a: u32, b:u32,
    }
    println!("{}", gcd(a, b));
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 || a == b {
        a
    } else if a > b {
        gcd(b, a % b)
    } else {
        gcd(a, b % a)
    }
}
