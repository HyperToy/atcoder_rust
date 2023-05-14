use proconio::*;

fn digit_sum(mut x: u32) -> u32 {
    let mut ret: u32 = 0;
    while x > 0 {
        ret += x % 10;
        x /= 10;
    }
    ret
}
fn is_harshad(x: u32) -> bool {
    x % digit_sum(x) == 0
}
fn main() {
    input! {
        n: u32,
    }
    println!("{}", if is_harshad(n) { "Yes" } else { "No" })
}
