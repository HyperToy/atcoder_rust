use itertools::Itertools;
use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        q: usize,
    }
    let qs: Vec<Option<usize>> = (0..q)
        .map(|_| {
            input! {
                t: usize,
            }
            match t {
                1 => {
                    input! {
                        x: usize,
                    }
                    Some(x)
                }
                2 => None,
                _ => unreachable!(),
            }
        })
        .collect();
    let mut bag = BinaryHeap::new();
    let mut answer = vec![];
    qs.iter().for_each(|q| match q {
        Some(x) => {
            bag.push(Reverse(x));
        }
        None => {
            if let Some(Reverse(x)) = bag.pop() {
                answer.push(x);
            }
        }
    });
    println!("{}", answer.iter().join("\n"));
}
