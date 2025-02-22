use proconio::*;

fn main() {
    input! {
        n: usize,
        p: [[i32; n]; n],
    }
    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if p[i][j] != 0 {
                x[i] = p[i][j];
                y[j] = p[i][j];
            }
        }
    }
    let mut answer = 0;
    for i in 0..n {
        for j in 0..i {
            answer += if x[j] > x[i] { 1 } else { 0 };
            answer += if y[j] > y[i] { 1 } else { 0 };
        }
    }
    println!("{}", answer);
}
