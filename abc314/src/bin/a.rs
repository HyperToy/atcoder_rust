use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let pi = String::from("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679");
    println!("{}", pi.chars().take(n + 2).join(""));
}
