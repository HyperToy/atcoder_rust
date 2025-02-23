use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! { mut s: Chars }
    let l = s.len();
    for i in (0..l - 1).rev() {
        if (s[i], s[i + 1]) == ('W', 'A') {
            s[i] = 'A';
            s[i + 1] = 'C';
        }
    }
    println!("{}", s.iter().join(""));
}
