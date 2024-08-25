use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [i32; n],
    }
    println!(
        "{}",
        a.iter()
            .dropping(n - k)
            .chain(a.iter().take(n - k))
            .join(" ")
    );
}
