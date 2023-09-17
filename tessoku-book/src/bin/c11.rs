use itertools::Itertools;
use proconio::*;
use std::collections::BinaryHeap;

// TLE
fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
    }
    let mut q = BinaryHeap::new();
    for i in 0..n {
        q.push((Rational::new(a[i], 1), i));
    }
    let mut answer = vec![0; n];
    for _ in 0..k {
        let v = q.pop().unwrap();
        answer[v.1] += 1;
        q.push((Rational::new(v.0.number, v.0.demon + 1), v.1));
    }
    println!("{}", answer.iter().join(" "));
}

#[derive(PartialEq, Eq)]
struct Rational {
    number: i64,
    demon: i64,
}
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.number * other.demon).partial_cmp(&(other.number * self.demon))
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.number * other.demon).cmp(&(other.number * self.demon))
    }
}
impl Rational {
    fn new(number: i64, demon: i64) -> Self {
        Self { number, demon }
    }
}
