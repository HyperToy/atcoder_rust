use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const DIR: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    let mut p1 = None;
    let mut p2 = None;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                s[i][j] = '.';
                if let Some(_) = p1 {
                    p2 = Some((i, j));
                } else {
                    p1 = Some((i, j));
                }
            }
        }
    }
    let (p1, p2) = (p1.unwrap(), p2.unwrap());

    let mut dist = vec![vec![vec![vec![std::i32::MAX; n]; n]; n]; n];
    let mut queue = VecDeque::new();
    dist[p1.0][p1.1][p2.0][p2.1] = 0;
    queue.push_back((p1, p2));
    dist[p2.0][p2.1][p1.0][p1.1] = 0;
    queue.push_back((p2, p1));

    while !queue.is_empty() {
        let (p1, p2) = queue.pop_front().unwrap();
        for k in 0..4 {
            let np1 = next_point(&s, p1, DIR[k]);
            let np2 = next_point(&s, p2, DIR[k]);
            if dist[np1.0][np1.1][np2.0][np2.1] <= dist[p1.0][p1.1][p2.0][p2.1] + 1 {
                continue;
            }
            dist[np1.0][np1.1][np2.0][np2.1] = dist[p1.0][p1.1][p2.0][p2.1] + 1;
            queue.push_back((np1, np2));
        }
    }

    let mut answer = std::i32::MAX;
    for i in 0..n {
        for j in 0..n {
            if dist[i][j][i][j] < answer {
                answer = dist[i][j][i][j];
            }
        }
    }
    println!("{}", if answer < std::i32::MAX { answer } else { -1 });
}

fn next_point(s: &Vec<Vec<char>>, p: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    let n = s.len() as isize;
    let ni = p.0 as isize + dir.0;
    let nj = p.1 as isize + dir.1;
    if ni < 0 || n <= ni || nj < 0 || n <= nj {
        // 移動先のマスが存在しない場合、移動しない
        p
    } else if s[ni as usize][nj as usize] == '#' {
        // 移動先のマスが空きマスでない場合、移動しない
        p
    } else {
        (ni as usize, nj as usize)
    }
}
