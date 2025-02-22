use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize, w: usize,
        s: (Usize1, Usize1),
        c: [Chars; h],
        x: Chars,
    }
    let mut pos = s;
    for d in x {
        let next = apply(pos, h, w, d);
        if c[next.0][next.1] == '.' {
            pos = next;
        }
    }
    println!("{} {}", pos.0 + 1, pos.1 + 1);
}

fn apply(pos: (usize, usize), h: usize, w: usize, dir: char) -> (usize, usize) {
    let (i, j) = pos;
    match dir {
        'L' => (i, if j > 0 { j - 1 } else { j }),
        'R' => (i, if j + 1 < w { j + 1 } else { j }),
        'U' => (if i > 0 { i - 1 } else { i }, j),
        'D' => (if i + 1 < h { i + 1 } else { i }, j),
        _ => unreachable!(),
    }
}
