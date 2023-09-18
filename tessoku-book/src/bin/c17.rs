use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }
    let mut former = VecDeque::new();
    let mut latter = VecDeque::new();

    for _ in 0..q {
        input! {
            t: char,
        }
        match t {
            'A' => {
                input! {
                    x: String,
                }
                latter.push_back(x);
                if former.len() < latter.len() {
                    former.push_back(latter.pop_front().unwrap());
                }
            }
            'B' => {
                input! {
                    x: String,
                }
                if former.len() == latter.len() {
                    former.push_back(x);
                } else {
                    latter.push_front(x);
                }
            }
            'C' => {
                former.pop_front();
                if former.len() < latter.len() {
                    former.push_back(latter.pop_front().unwrap());
                }
            }
            'D' => {
                println!("{}", former.front().unwrap());
            }
            _ => unreachable!(),
        }
    }
}
