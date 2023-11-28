use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    a.reverse();
    a.dedup();
    println!("{}", a[1]);
}
