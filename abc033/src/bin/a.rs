use proconio::*;

fn main() {
    input! {
        s: u32,
    }
    println!(
        "{}",
        if s % 1111 == 0 {
            "SAME"
        } else {
            "DIFFERENT"
        }
    )
}
