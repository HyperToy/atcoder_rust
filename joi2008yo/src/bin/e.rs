use proconio::*;

fn main() {
    input! {
        h: usize, w: usize,
        c: [[usize; w]; h],
    }
    let mut answer = 0;
    for mask in 0..(1 << h) {
        let mut now = 0;
        for j in 0..w {
            let mut black_count = 0;
            for i in 0..h {
                if ((mask >> i) & 1 == 1) ^ (c[i][j] == 1) {
                    black_count += 1;
                }
            }
            now += black_count.max(h - black_count);
        }
        answer = answer.max(now);
    }
    println!("{}", answer);
}
