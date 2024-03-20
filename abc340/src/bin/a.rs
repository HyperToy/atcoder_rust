use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b, d): (i32, i32, i32),
    }
    println!("{}", (a..=b).filter(|v| (*v - a) % d == 0).join(" "));
}
