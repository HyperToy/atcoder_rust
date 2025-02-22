use core::cmp::Reverse;
use proconio::*;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }
    let mut priority_queue = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: i32,
                }
                priority_queue.push(Reverse(x));
            }
            2 => {
                println!("{}", priority_queue.peek().unwrap().0);
            }
            3 => {
                priority_queue.pop();
            }
            _ => unreachable!(),
        }
    }
}
