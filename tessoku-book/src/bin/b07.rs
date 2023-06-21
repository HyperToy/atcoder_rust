use proconio::*;

fn main() {
    input! {
        t: usize,
        n: usize,
        qs: [(usize, usize); n],
    }
    let mut sum = vec![0; t + 1];
    for (l, r) in qs {
        sum[l] += 1;
        sum[r] -= 1;
    }
    for i in 0..t {
        println!("{}", sum[i]);
        sum[i + 1] += sum[i];
    }
}
