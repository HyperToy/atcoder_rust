use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    println!(
        "{}",
        1 + s.into_iter().find_position(|c| c.is_uppercase()).unwrap().0
    );
}
