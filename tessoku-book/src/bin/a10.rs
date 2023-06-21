use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        ps: [(usize, usize); d],
    }
    let mut left = vec![0; n + 2];
    let mut right = vec![0; n + 2];
    for i in 1..=n {
        left[i] = a[i - 1];
        left[i] = left[i].max(left[i - 1]);
    }
    for i in (1..=n).rev() {
        right[i] = a[i - 1];
        right[i] = right[i].max(right[i + 1]);
    }
    for (l, r) in ps {
        println!("{}", left[l - 1].max(right[r + 1]));
    }
}
