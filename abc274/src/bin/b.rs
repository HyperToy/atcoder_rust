use proconio::*;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    for j in 0..w {
        let mut x: u32 = 0;
        for i in 0..h {
            x += if c[i][j] == '.' {
                0
            } else {
                1
            }
        }
        println!("{}", x);
    }
}
