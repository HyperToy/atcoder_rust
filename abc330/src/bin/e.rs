use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize, q: usize,
        mut a: [usize; n],
        queries: [(Usize1, usize); q],
    }
    let mut count = vec![0; n + 1];
    let mut zero = BinaryHeap::new();
    for i in 0..n {
        if a[i] <= n {
            count[a[i]] += 1;
        }
    }
    for i in 0..=n {
        if count[i] == 0 {
            zero.push(Reverse(i));
        }
    }
    for (i, x) in queries {
        if a[i] <= n {
            count[a[i]] -= 1;
            if count[a[i]] == 0 {
                zero.push(Reverse(a[i]));
            }
        }
        a[i] = x;
        if a[i] <= n {
            count[x] += 1;
        }
        loop {
            let x: usize = zero.peek().unwrap().0;
            if count[x] > 0 {
                zero.pop();
            } else {
                println!("{}", x);
                break;
            }
        }
        // eprintln!("{:?}", zero);
    }
}
