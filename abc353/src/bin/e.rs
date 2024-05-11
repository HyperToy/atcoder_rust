use proconio::{input, marker::Chars};
use std::collections::HashMap;

// TLE
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        for j in 0..s[i].len() {
            *(count.entry(&s[i][0..=j]).or_insert(0)) += 1;
        }
    }
    let mut answer = 0i64;
    for (_k, v) in count {
        answer += v * (v - 1) / 2;
    }
    println!("{}", answer);
}
