use proconio::*;

fn main() {
    input! {
        h: usize, w: usize,
        x: [[i32; w]; h],
        q: usize,
        ps: [(usize, usize, usize, usize); q],
    }
    let mut sum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            sum[i + 1][j + 1] = x[i][j];
        }
    }
    for i in 0..=h {
        for j in 1..=w {
            sum[i][j] += sum[i][j - 1];
        }
    }
    for j in 0..=w {
        for i in 1..=h {
            sum[i][j] += sum[i - 1][j];
        }
    }
    for (a, b, c, d) in ps {
        println!(
            "{}",
            sum[c][d] - sum[c][b - 1] - sum[a - 1][d] + sum[a - 1][b - 1]
        );
    }
}
