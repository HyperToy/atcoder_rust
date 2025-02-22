use proconio::{marker::Bytes, *};
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        mut s: [Bytes; n],
    }
    for i in 0..n {
        for j in 0..s[i].len() / 2 {
            match s[i][j].cmp(&s[i][s[i].len() - 1 - j]) {
                Ordering::Less => {
                    break;
                }
                Ordering::Equal => {
                    continue;
                }
                Ordering::Greater => {
                    s[i].reverse();
                    break;
                }
            }
        }
    }
    s.sort();
    s.dedup();
    println!("{}", s.len());
}
