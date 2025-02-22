use proconio::*;

fn main() {
    input! {
        a: i64, b: i64, c: i64,
    }
    println!("{}", if a + b + c == 0 { "Yes" } else { "No" });
}
