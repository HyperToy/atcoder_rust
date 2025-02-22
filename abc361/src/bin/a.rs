use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: usize, x: i32,
        mut a: [i32; n],
    }
    a.insert(k, x);
    println!("{}", a.iter().join(" "));
}
