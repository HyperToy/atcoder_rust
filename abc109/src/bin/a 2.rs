use proconio::*;

fn main() {
    input! {
        a: u8,
        b: u8,
    }
    println!(
        "{}",
        if (a * b) % 2 == 1 {
            "Yes"
        } else {
            "No"
        }
    );
}
