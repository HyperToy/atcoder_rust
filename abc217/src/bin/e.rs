use proconio::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }
    let mut v = VecDeque::new();
    let mut pq: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: u32,
        }
        match t {
            1 => {
                input! {
                    x: u32,
                }
                v.push_back(x);
            }
            2 => {
                if let Some(x) = pq.pop() {
                    println!("{}", x.0);
                } else {
                    println!("{}", v.pop_front().unwrap());
                }
            }
            3 => {
                while !v.is_empty() {
                    let x = v.pop_back().unwrap();
                    pq.push(Reverse(x));
                }
            }
            _ => (),
        }
    }
}
