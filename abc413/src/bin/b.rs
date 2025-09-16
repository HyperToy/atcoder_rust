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
            .permutations(2)
            .map(|ss| format!("{}{}", ss[0], ss[1]))
            .sorted()
            .dedup()
            .count()
    );
}
