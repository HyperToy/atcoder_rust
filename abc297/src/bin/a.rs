use proconio::*;

fn main() {
    input! {
        n: usize, d: i32,
        t: [i32; n],
    }
    let mut answer = -1;
    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            answer = t[i];
            break;
        }
    }
    println!("{}", answer);
}
