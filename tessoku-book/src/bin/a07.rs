use proconio::*;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut sum = vec![0; d + 2];
    for (l, r) in lr {
        sum[l] += 1;
        sum[r + 1] -= 1;
    }
    for i in 0..d {
        sum[i + 1] += sum[i];
        println!("{}", sum[i + 1]);
    }
}
