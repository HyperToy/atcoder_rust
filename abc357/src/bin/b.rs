use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let l = s.len();
    let upper = s.iter().filter(|&&c| c.is_uppercase()).count();
    let s = s;
    println!(
        "{}",
        s.into_iter()
            .map(|c| {
                if upper > l - upper {
                    if c.is_uppercase() {
                        c.to_string()
                    } else {
                        c.to_uppercase().to_string()
                    }
                } else {
                    if c.is_uppercase() {
                        c.to_lowercase().to_string()
                    } else {
                        c.to_string()
                    }
                }
            })
            .join("")
    );
}
