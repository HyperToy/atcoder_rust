use proconio::*;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        k: i64,
    }
    println!(
        "{}",
        (a - b) * if k % 2 == 1 {
            -1
        } else {
            1
        }
    );
}
