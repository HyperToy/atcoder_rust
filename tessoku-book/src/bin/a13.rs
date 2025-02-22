use proconio::*;

fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    let mut r = 0;
    let mut answer = 0;
    for l in 0..n {
        while r < n && a[r] - a[l] <= k {
            r += 1;
        }
        answer += r - l - 1;
    }
    println!("{}", answer);
}
