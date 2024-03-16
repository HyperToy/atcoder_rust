use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut answer = n * (n - 1) / 2;
    let mut count = HashMap::new();
    for c in s {
        *(count.entry(c).or_insert(0)) += 1;
    }
    let mut any = 0;
    for (_, v) in count {
        answer -= v * (v - 1) / 2;
        if v > 1 {
            any = 1;
        }
    }
    answer += any;
    println!("{}", answer);
}
