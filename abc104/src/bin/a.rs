use proconio::*;

fn main() {
    input! {
        r: u32,
    }
    println!(
        "{}",
        if r < 1200 {
            "ABC"
        } else if r  < 2800 {
            "ARC"
        } else {
            "AGC"
        }
    )
}
