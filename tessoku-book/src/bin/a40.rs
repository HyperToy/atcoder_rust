use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut lens = vec![0i64; 101];
    for i in 0..n {
        lens[a[i]] += 1;
    }
    let answer = lens.iter().map(|x| x * (x - 1) * (x - 2) / 6).sum::<i64>();
    println!("{}", answer);
}
