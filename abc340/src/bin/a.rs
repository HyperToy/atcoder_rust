use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b, d): (i32, i32, i32),
    }
    println!("{}", (0..=(b - a) / d).map(|i| a + i * d).join(" "));
}
