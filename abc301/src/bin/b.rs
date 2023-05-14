use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    for i in 0..n - 1 {
        print!("{} ", a[i]);
        if a[i] < a[i + 1] {
            for x in a[i] + 1..a[i + 1] {
                print!("{} ", x);
            }
        } else {
            for x in (a[i + 1] + 1..a[i]).rev() {
                print!("{} ", x);
            }
        }
    }
    println!("{}", a[n - 1]);
}
