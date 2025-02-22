use itertools::Itertools;
use proconio::input;

// refactor
fn main() {
    input! {
        n: usize,
        a: [i32; 7 * n],
    }
    let mut b = vec![0; n];
    for i in 0..n {
        for j in 0..7 {
            b[i] += a[7 * i + j];
        }
    }
    println!("{}", b.iter().join(" "));
}
