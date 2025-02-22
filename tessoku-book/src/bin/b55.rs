use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: usize,
        qs: [(usize, i32); q],
    }
    let mut set = BTreeSet::new();
    for (t, x) in qs {
        match t {
            1 => {
                set.insert(x);
            }
            2 => {
                let mut answer = std::i32::MAX;
                match set.range(x..).next() {
                    Some(y) => {
                        answer = answer.min((x - y).abs());
                    }
                    None => (),
                }
                match set.range(..=x).next_back() {
                    Some(y) => {
                        answer = answer.min((x - y).abs());
                    }
                    None => (),
                }
                println!("{}", if answer < std::i32::MAX { answer } else { -1 });
            }
            _ => unreachable!(),
        }
    }
}
