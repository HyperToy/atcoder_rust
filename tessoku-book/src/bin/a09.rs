use proconio::*;

fn main() {
    input! {
        h: usize, w: usize, n: usize,
        snows: [(usize, usize, usize, usize); n],
    }
    let mut sum = vec![vec![0; w + 2]; h + 2];
    for (a, b, c, d) in snows {
        sum[a][b] += 1;
        sum[a][d + 1] -= 1;
        sum[c + 1][b] -= 1;
        sum[c + 1][d + 1] += 1;
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

    for i in 1..=h {
        for j in 1..=w {
            if j == w {
                println!("{}", sum[i][j]);
            } else {
                print!("{} ", sum[i][j]);
            }
        }
    }
}
