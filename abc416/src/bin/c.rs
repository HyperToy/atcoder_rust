use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, k: usize, x: Usize1,
        s: [String; n],
    }
    let answer = (0..k)
        .map(|_| 0..n)
        .multi_cartesian_product()
        .map(|p| p.iter().map(|i| &s[*i]).join(""))
        .sorted()
        .nth(x)
        .unwrap();
    println!("{}", answer);
}
