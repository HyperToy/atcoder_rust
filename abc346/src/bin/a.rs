use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut b = Vec::new();
    for i in 1..n {
        b.push(a[i - 1] * a[i]);
    }
    println!("{}", b.iter().join(" "));
}
