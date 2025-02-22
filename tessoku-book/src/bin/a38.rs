use proconio::{marker::Usize1, *};

fn main() {
    input! {
        d: usize, n: usize,
        conditions: [(Usize1, Usize1, i32); n],
    }
    let mut limits = vec![24; d];
    for (l, r, h) in conditions {
        for i in l..=r {
            limits[i] = limits[i].min(h);
        }
    }
    let answer = limits.iter().sum::<i32>();
    println!("{}", answer);
}
