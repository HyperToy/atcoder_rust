use proconio::*;

fn main() {
    input! {
        n: usize, q: usize,
        a: [i32; n],
        lr: [(usize, usize); q],
    }
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }
    for (l, r) in lr {
        println!("{}", sum[r] - sum[l - 1]);
    }
}
