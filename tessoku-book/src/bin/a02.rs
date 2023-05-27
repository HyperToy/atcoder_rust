use proconio::*;

fn main() {
    input! {
        n: usize, x: u32,
        a: [u32; n],
    }
    println!(
        "{}",
        if a.iter().any(|value| value == &x) {
            "Yes"
        } else {
            "No"
        }
    );
}
