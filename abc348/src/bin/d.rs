use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{HashMap, VecDeque};

const DY: [isize; 4] = [0, 1, 0, -1];
const DX: [isize; 4] = [1, 0, -1, 0];

fn main() {
    input! {
        (h, w): (usize, usize),
        mut a: [Chars; h],
        n: usize,
        potions: [((Usize1, Usize1), i32); n],
    }
    let potions = potions.into_iter().collect::<HashMap<_, _>>();
    let (sy, sx, ty, tx) = {
        let (mut sy, mut sx, mut ty, mut tx) = (0, 0, 0, 0);
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == 'S' {
                    a[i][j] = '.';
                    (sy, sx) = (i, j);
                }
                if a[i][j] == 'T' {
                    a[i][j] = '.';
                    (ty, tx) = (i, j);
                }
            }
        }
        (sy, sx, ty, tx)
    };
    let mut max_energy = vec![vec![std::i32::MIN; w]; h];
    let mut queue = VecDeque::new();
    let initial_energy = 0;
    max_energy[sy][sx] = initial_energy;
    queue.push_back(((sy, sx), initial_energy));
    while !queue.is_empty() {
        let now = queue.pop_front().unwrap();
        let (py, px) = now.0;
        let mut e = now.1;
        if let Some(ne) = potions.get(&(py, px)) {
            if *ne > e && *ne > max_energy[py][px] {
                e = *ne;
                max_energy[py][px] = e;
                queue.push_back(((py, px), e));
                continue;
            }
        }
        if e == 0 {
            continue;
        }
        for k in 0..4 {
            let (ny, nx) = (py as isize + DY[k], px as isize + DX[k]);
            let ne = e - 1;
            if ny < 0 || h as isize <= ny || nx < 0 || w as isize <= nx {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if a[ny][nx] == '#' {
                continue;
            }
            if max_energy[ny][nx] >= ne {
                continue;
            }
            max_energy[ny][nx] = ne;
            queue.push_back(((ny, nx), ne));
        }
    }
    println!(
        "{}",
        if max_energy[ty][tx] > std::i32::MIN {
            "Yes"
        } else {
            "No"
        }
    );
}
