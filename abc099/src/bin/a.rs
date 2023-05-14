use proconio::*;

fn main() {
    input! {
        n: u32,
    }
    println!(
        "{}",
        if n < 1000 {
            "ABC"
        } else {
            "ABD"
        }
    )
}
