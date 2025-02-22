use itertools::Itertools;
use proconio::{marker::Usize1, *};

fn main() {
    input! {
        t: usize,
        pqr: [(Usize1, Usize1, Usize1); t]
    }
    println!("{}", solve(pqr).iter().join(" "));
}

fn solve(ops: Vec<(usize, usize, usize)>) -> Vec<char> {
    let t = ops.len();
    // let mut x = vec![0; 20];
    let answer = vec!['A'; t];
    answer
}
