use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let n = h.min(w);
    let mut answer = vec![0; n + 1];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }
            for l in 1.. {
                let nbs = neighbors(i, j, l, h, w);
                if nbs.len() != 4 || nbs.iter().any(|&(y, x)| s[y][x] != '#') {
                    answer[l - 1] += 1;
                    break;
                }
            }
        }
    }
    println!("{}", answer.iter().skip(1).join(" "));
}

fn neighbors(i: usize, j: usize, l: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    [(1, 1), (1, -1), (-1, -1), (-1, 1)]
        .iter()
        .map(|&(dy, dx)| (i as isize + dy * l as isize, j as isize + dx * l as isize))
        .filter_map(|(ni, nj)| {
            if 0 <= ni && ni < h as isize && 0 <= nj && nj < w as isize {
                Some((ni as usize, nj as usize))
            } else {
                None
            }
        })
        .collect_vec()
}
