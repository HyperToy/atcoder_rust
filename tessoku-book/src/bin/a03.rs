use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize, k: u32,
        p: [u32; n],
        q: [u32; n],
    }
    println!(
        "{}",
        if p.iter().cartesian_product(q).any(|(a, b)| a + b == k) {
            "Yes"
        } else {
            "No"
        }
    );
}
