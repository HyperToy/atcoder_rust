use proconio::{marker::Usize1, *};
use std::cmp::Ordering;

fn main() {
    input! {
        d: usize,
        x: i32,
        a: [i32; d - 1],
        q: usize,
        qs: [(Usize1,Usize1); q],

    }
    let mut sum = vec![0; d];
    sum[0] = x;
    for i in 1..d {
        sum[i] = sum[i - 1] + a[i - 1];
    }
    for (s, t) in qs {
        println!(
            "{}",
            match sum[s].cmp(&sum[t]) {
                Ordering::Greater => (s + 1).to_string(),
                Ordering::Less => (t + 1).to_string(),
                Ordering::Equal => "Same".to_string(),
            }
        );
    }
}
