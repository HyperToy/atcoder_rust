use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, q: usize,
    }
    let mut waiting = (0..n).collect::<BTreeSet<_>>();
    let mut called = BTreeSet::new();
    let mut answer = vec![];
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                called.insert(waiting.pop_first().unwrap());
            }
            2 => {
                input! { x: Usize1 }
                called.remove(&x);
            }
            3 => {
                answer.push(*called.first().unwrap());
            }
            _ => unreachable!(),
        }
    }
    println!("{}", answer.iter().map(|&x| x + 1).join("\n"));
}
