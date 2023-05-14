use proconio::*;

fn main() {
    input! {
        mut a: [i64; 3],
    }
    a.sort();
    println!(
        "{}",
        if a[0] + a[1] < a[2] {
            -1
        } else {
            a[2]
        }
    )
}
