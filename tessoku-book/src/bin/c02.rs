use proconio::*;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    a.reverse();
    println!("{}", a[0] + a[1]);
}
