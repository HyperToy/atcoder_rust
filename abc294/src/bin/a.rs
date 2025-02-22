use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    println!("{}", a.into_iter().filter(|&x| x % 2 == 0).join(" "));
}
