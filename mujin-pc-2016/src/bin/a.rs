use proconio::*;

fn main() {
    input! {
        c: char
    }
    println!(
        "{}",
        if "KLOP".chars().any(|d| d == c) {
            "Right"
        } else {
            "Left"
        }
    )
}
