use proconio::*;

fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    // [l, r) の和は sum[r] - sum[l];
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut r = 0;
    let mut answer = 0;
    for l in 0..n {
        while r <= n && sum[r] - sum[l] <= k {
            r += 1;
        }
        answer += r - l - 1;
    }
    println!("{}", answer);
}
