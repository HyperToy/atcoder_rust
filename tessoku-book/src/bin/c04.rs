use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: u64,
    }
    let mut divisors = Vec::new();
    let mut x = 1;
    while x * x <= n {
        if n % x == 0 {
            divisors.push(x);
            if x != n / x {
                divisors.push(n / x);
            }
        }
        x += 1;
    }
    divisors.sort();
    println!("{}", divisors.iter().join(" "));
}
