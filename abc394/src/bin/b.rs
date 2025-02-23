use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    println!(
        "{}",
        s.iter().sorted_by(|a, b| a.len().cmp(&b.len())).join("")
    );
}
