use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const DX: [isize; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
const DY: [isize; 8] = [0, -1, -1, -1, 0, 1, 1, 1];
fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }
    let mut queue = VecDeque::new();
    let mut seen = vec![vec![false; w]; h];
    let mut answer = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' || seen[i][j] {
                continue;
            }
            answer += 1;
            queue.push_back((i, j));
            seen[i][j] = true;
            while let Some((x, y)) = queue.pop_front() {
                for k in 0..8 {
                    let nx = x as isize + DX[k];
                    let ny = y as isize + DY[k];
                    if nx < 0 || h as isize <= nx || ny < 0 || w as isize <= ny {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if s[nx][ny] != '#' || seen[nx][ny] {
                        continue;
                    }
                    queue.push_back((nx, ny));
                    seen[nx][ny] = true;
                }
            }
        }
    }
    println!("{}", answer);
}
