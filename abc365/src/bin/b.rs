use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    println!(
        "{}",
        a.iter()
            .position(|x| x == a.iter().sorted().rev().nth(1).unwrap())
            .unwrap()
            + 1
    );
}
