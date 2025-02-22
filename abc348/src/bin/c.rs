use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        ac: [(i32, i32); n],
    }
    let mut map = HashMap::new();
    for (a, c) in ac {
        let min = map.entry(c).or_insert(std::i32::MAX);
        if *min > a {
            *min = a;
        }
    }
    let mut answer = std::i32::MIN;
    for (_, v) in map {
        if answer < v {
            answer = v;
        }
    }
    println!("{}", answer);
}
