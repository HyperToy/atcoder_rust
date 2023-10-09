use proconio::*;

fn main() {
    input! {
        mut n: i32,
    }
    println!("{}", if is_decreasing_number(n) { "Yes" } else { "No" });
}

fn is_decreasing_number(n: i32) -> bool {
    let mut ok = true;
    let mut num = n;
    while num >= 10 {
        ok &= num / 10 % 10 > num % 10;
        num /= 10;
    }
    ok
}
