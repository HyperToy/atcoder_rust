use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut count = BTreeMap::new();
    for c in s {
        *(count.entry(c).or_insert(0)) += 1;
    }
    // refactor
    let mut answer = '.';
    let mut max = 0;
    for (c, x) in count {
        if x > max {
            max = x;
            answer = c;
        }
    }
    println!("{}", answer);
}
