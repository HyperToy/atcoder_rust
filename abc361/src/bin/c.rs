use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
    }
    let a = a.into_iter().sorted().collect_vec();
    let l = n - k;
    let mut q = VecDeque::new();
    let mut answer = std::i64::MAX;
    for x in a {
        q.push_back(x);
        if q.len() == l {
            answer = answer.min(q.back().unwrap() - q.front().unwrap());
            q.pop_front();
        }
    }
    println!("{}", answer);
}
