use proconio::*;

fn main() {
    input! {
        n: usize,
    }
    let a = fibonacci(n, 1000000000 + 7);
    println!("{}", a[n]);
}
fn fibonacci(max: usize, modulo: u64) -> Vec<u64> {
    let mut res = vec![0; max + 1];
    res[1] = 1;
    res[2] = 1;
    for i in 3..=max {
        res[i] = (res[i - 1] + res[i - 2]) % modulo;
    }
    res
}
