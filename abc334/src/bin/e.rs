use ac_library::ModInt998244353 as Mint;
use proconio::{input, marker::Chars};
use std::collections::{HashSet, VecDeque};

const DYX: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }
    // 各セルに id を振る。
    // 緑 (#) 同士で連結している時、同じ id (>0)
    // 0 のとき、未達 or 赤 (.)
    let mut field = vec![vec![0; w]; h];
    let mut q = VecDeque::new();
    let mut id = 0;
    for i in 0..h {
        for j in 0..w {
            if field[i][j] > 0 || s[i][j] != '#' {
                continue;
            }
            id += 1;
            field[i][j] = id;
            q.push_back((i, j));
            while !q.is_empty() {
                let (y, x) = q.pop_front().unwrap();
                for (dy, dx) in DYX {
                    let (ny, nx) = (y as isize + dy, x as isize + dx);
                    if ny < 0 || h as isize <= ny || nx < 0 || w as isize <= nx {
                        continue;
                    }
                    let (ny, nx) = (ny as usize, nx as usize);
                    if field[ny][nx] > 0 || s[ny][nx] != '#' {
                        continue;
                    }
                    field[ny][nx] = id;
                    q.push_back((ny, nx));
                }
            }
        }
    }
    let num_component = id; // 連結成分の個数
    let mut sum = 0;
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if field[i][j] > 0 {
                continue;
            }
            let mut set = HashSet::new();
            for (dy, dx) in DYX {
                let (ny, nx) = (i as isize + dy, j as isize + dx);
                if ny < 0 || h as isize <= ny || nx < 0 || w as isize <= nx {
                    continue;
                }
                let (ny, nx) = (ny as usize, nx as usize);
                if field[ny][nx] == 0 {
                    continue;
                }
                set.insert(field[ny][nx]);
            }
            sum += num_component - (set.len() as i32 - 1);
            count += 1;
        }
    }
    println!("{}", Mint::new(sum) / count);
}
