use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }
    let mut set = HashSet::new();
    let n = s.len();
    for j in 0..n {
        for i in 0..=j {
            let mut now = Vec::new();
            for k in i..=j {
                now.push(s[k]);
            }
            set.insert(now);
        }
    }
    println!("{}", set.len());
}
