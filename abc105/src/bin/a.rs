use proconio::*;

fn main() {
    input! {
        (n, k): (i32, i32),
    }
    println!("{}", if n % k == 0 { 0 } else { 1 });
}
