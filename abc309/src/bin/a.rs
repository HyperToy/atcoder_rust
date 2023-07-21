use proconio::*;

fn main() {
    input! {
        a: usize, b: usize,
    }
    println!(
        "{}",
        if a % 3 != 0 && a + 1 == b {
            "Yes"
        } else {
            "No"
        }
    );
}
