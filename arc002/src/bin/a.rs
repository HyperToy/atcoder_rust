use proconio::*;

fn main() {
    input! {
        y: u32,
    }
    println!(
        "{}",
        if y % 400 == 0 {
            "YES"
        } else if y % 100 == 0 {
            "NO"
        } else if y % 4 == 0 {
            "YES"
        } else {
            "NO"
        }
    )
}
