use proconio::{marker::Chars, *};
use std::collections::VecDeque;

const DX: [isize; 4] = [1, 0, -1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];
fn main() {
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    }
    let mut stopped = vec![vec![false; m]; n];
    let mut passed = vec![vec![false; m]; n];
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    stopped[1][1] = true;
    queue.push_back((1, 1));
    while !queue.is_empty() {
        let (now_x, now_y) = queue.pop_front().unwrap();
        for k in 0..4 {
            let (mut x, mut y) = (now_x, now_y);
            loop {
                if s[x as usize][y as usize] == '#' {
                    // 行き着いた先が岩だった場合
                    x -= DX[k];
                    y -= DY[k];
                    if !stopped[x as usize][y as usize] {
                        stopped[x as usize][y as usize] = true;
                        queue.push_back((x, y));
                    }
                    break;
                }
                passed[x as usize][y as usize] = true;
                x += DX[k];
                y += DY[k];
            }
        }
    }
    let mut answer = 0;
    for i in 0..n {
        for j in 0..m {
            if stopped[i][j] || passed[i][j] {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
