use proconio::*;

fn main() {
    input! {
        n: usize, k: u32,
        p: [u32; n],
        q: [u32; n],
    }
    println!(
        "{}",
        if p.iter().any(|a| q.iter().any(|b| a + b == k)) {
            "Yes"
        } else {
            "No"
        }
    );
}
