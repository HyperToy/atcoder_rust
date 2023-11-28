use proconio::{marker::Usize1, *};
use std::{cmp::Reverse, collections::BinaryHeap};

// refactor
fn main() {
    input! {
        n: usize, m: usize,
        a: [Usize1; m],
    }
    let mut votes = BinaryHeap::new();
    let mut count = vec![0; n];
    for x in a {
        count[x] += 1;
        votes.push((count[x], Reverse(x)));
        println!("{}", votes.peek().unwrap().1 .0 + 1);
    }
}
