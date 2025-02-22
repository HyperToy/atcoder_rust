use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
    }
    let k = k + 1;
    let mut s = BTreeSet::new();
    s.insert(0);
    let mut answer = vec![];
    while answer.len() < k {
        let v = s.pop_first().unwrap();
        answer.push(v);
        for &a in &a {
            s.insert(v + a);
        }
    }
    println!("{}", answer[k - 1]);
}
