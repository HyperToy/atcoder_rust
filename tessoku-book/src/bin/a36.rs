use proconio::*;

fn main() {
    input! {
        n: i32, k: i32,
    }
    println!(
        "{}",
        if 2 * (n - 1) <= k && k % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
