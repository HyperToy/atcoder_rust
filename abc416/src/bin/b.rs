use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    let t = (0..n)
        .map(|i| {
            if s[i] == '#' {
                '#'
            } else if i == n - 1 || s[i] == '.' && s[i + 1] == '#' {
                'o'
            } else {
                '.'
            }
        })
        .join("");
    println!("{}", t);
}
