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
            .map(yes_no)
            .join("\n")
    );
}

fn yes_no(b: bool) -> String {
    match b {
        true => "Yes".to_string(),
        false => "No".to_string(),
    }
}
