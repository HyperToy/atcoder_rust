use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let radix = 5;
    println!(
        "{}",
        to_string_radix(n - 1, radix)
            .chars()
            .map(|c| c.to_digit(radix).unwrap())
            .map(|d| d * 2)
            .join("")
    );
}
fn to_string_radix(mut n: u64, radix: u32) -> String {
    let mut res = Vec::new();
    while n > 0 {
        res.push(n % radix as u64);
        n /= radix as u64;
    }
    res.reverse();
    if res.is_empty() {
        res.push(0);
    }
    res.iter().join("")
}
