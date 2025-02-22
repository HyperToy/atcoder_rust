use proconio::*;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, d: usize,
        mut tasks: [(usize, i32); n],
    }
    tasks.sort();
    let mut pq = BinaryHeap::new();
    let mut j = 0;
    let mut answer = 0;
    for i in 1..=d {
        while j < n && tasks[j].0 <= i {
            pq.push(tasks[j].1);
            j += 1;
        }
        if let Some(x) = pq.pop() {
            answer += x;
        }
    }
    println!("{}", answer);
}
