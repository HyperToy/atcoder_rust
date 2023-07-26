use proconio::*;

// todo
fn main() {
    input! {
        n: usize,
        p: [[i32; n]; n],
    }
    let mut answer = 0;
    for i in 0..n {
        for j in 0..n {
            if p[i][j] != 0 {
                answer += (i as i32 - p[i][j] + 1).abs() + (j as i32 - p[i][j] + 1).abs();
            }
        }
    }
    println!("{}", answer / 2);
}
