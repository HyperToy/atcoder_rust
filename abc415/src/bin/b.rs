use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    println!(
        "{}",
        s.into_iter()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .into_iter()
            .map(|chunk| format!("{},{}", chunk[0].0 + 1, chunk[1].0 + 1))
            .join("\n")
    );
}
