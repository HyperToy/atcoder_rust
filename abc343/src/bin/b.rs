use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }
    for i in 0..n {
        println!(
            "{}",
            a[i].iter()
                .enumerate()
                .filter(|(_, f)| **f == 1)
                .map(|(v, _)| v + 1)
                .join(" ")
        );
    }
}
