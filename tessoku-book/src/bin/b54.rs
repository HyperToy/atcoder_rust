use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        // map[a[i]] += 1;
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let mut answer = 0i64;
    for (_, v) in map {
        answer += v * (v - 1) / 2;
    }
    println!("{}", answer);
}
