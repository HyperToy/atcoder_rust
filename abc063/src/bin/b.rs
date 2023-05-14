use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }
    let mut ans = "yes";
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
        if *count > 1 {
            ans = "no";
        }
    }
    println!("{}", ans);
}
