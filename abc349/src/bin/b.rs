use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars
    }
    let mut count_char = HashMap::new();
    for c in s {
        *(count_char.entry(c).or_insert(0)) += 1;
    }
    let mut count_appearance = HashMap::new();
    for (_, x) in count_char {
        *(count_appearance.entry(x).or_insert(0)) += 1;
    }
    println!(
        "{}",
        if count_appearance.iter().all(|(_, x)| *x == 2) {
            "Yes"
        } else {
            "No"
        }
    );
}
