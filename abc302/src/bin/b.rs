use proconio::{marker::Chars, *};

const DX: [isize; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
const DY: [isize; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

fn main() {
    input! {
        h: isize, w: isize,
        s: [Chars; h],
    }
    let target = vec!['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            for d in 0..8 {
                let mut nx = i;
                let mut ny = j;
                let mut ok = false;
                for k in 0..5 {
                    if nx < 0 || h <= nx || ny < 0 || w <= ny {
                        break;
                    }
                    if target[k] != s[nx as usize][ny as usize] {
                        break;
                    }
                    nx += DX[d];
                    ny += DY[d];
                    if k == 4 {
                        ok = true;
                    }
                }
                if ok {
                    let mut x = i;
                    let mut y = j;
                    for _ in 0..5 {
                        println!("{} {}", x + 1, y + 1);
                        x += DX[d];
                        y += DY[d];
                    }
                }
            }
        }
    }
}
