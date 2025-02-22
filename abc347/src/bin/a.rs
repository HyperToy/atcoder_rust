use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: i32,
        a: [i32; n],
    }
    println!(
        "{}",
        a.iter().filter(|x| *x % k == 0).map(|x| x / k).join(" ")
    );
}
