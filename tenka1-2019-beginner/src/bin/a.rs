use proconio::*;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    println!(
        "{}",
        if (c - a) * (b - c) >= 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
