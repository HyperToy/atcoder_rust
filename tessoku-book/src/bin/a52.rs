use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: String,
                }
                queue.push_back(x);
            }
            2 => {
                println!("{}", queue.front().unwrap());
            }
            3 => {
                queue.pop_front();
            }
            _ => unreachable!(),
        }
    }
}
