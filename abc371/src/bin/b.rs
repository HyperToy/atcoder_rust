use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize, m: usize,
        ab: [(Usize1, char); m],
    }
    println!(
        "{}",
        ab.iter()
            .enumerate()
            .map(|(i, (a, b))| *b == 'M' && ab.iter().take(i).all(|(pa, pb)| pa != a || *pb == 'F'))
            .map(|b| if b { "Yes" } else { "No" })
            .join("\n")
    );
}
