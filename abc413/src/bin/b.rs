use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    println!(
        "{}",
        s.iter()
            .enumerate()
            .cartesian_product(s.iter().enumerate())
            .filter(|((i, _), (j, _))| i != j)
            .map(|((_, s), (_, t))| format!("{}{}", s, t))
            .sorted()
            .dedup()
            .count()
    );
}
