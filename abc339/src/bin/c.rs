use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut sum = 0;
    let mut min = 0;
    for i in 0..n {
        sum += a[i];
        if min > sum {
            min = sum;
        }
    }
    println!("{}", -min + sum);
}
