use proconio::{
    marker::{Chars, Usize1},
    *,
};
use std::collections::VecDeque;

const DX: [isize; 4] = [1, 0, -1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

fn main() {
    input! {
        h: usize, w: usize,
        sy: Usize1, sx: Usize1,
        gy: Usize1, gx: Usize1,
        c: [Chars; h],
    }
    let mut dist = vec![vec![1_000_000_000; w]; h];
    let mut q = VecDeque::new();

    dist[sy][sx] = 0;
    q.push_back((sy, sx));
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        let (y, x) = (p.0, p.1);
        for k in 0..4 {
            let (ny, nx) = ((y as isize + DY[k]) as usize, (x as isize + DX[k]) as usize);
            if c[ny][nx] == '#' {
                continue;
            }
            if dist[ny][nx] <= dist[y][x] + 1 {
                continue;
            }
            dist[ny][nx] = dist[y][x] + 1;
            q.push_back((ny, nx));
        }
    }
    println!("{}", dist[gy][gx]);
}
