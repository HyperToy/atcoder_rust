use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize, _n: usize,
        t: Chars,
        s: [Chars; h],
    }
    let moves = t
        .iter()
        .map(|c| match *c {
            'L' => (0, -1),
            'R' => (0, 1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut answer = 0;
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if check(&s, &moves, (i, j)) {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
fn check(field: &Vec<Vec<char>>, moves: &Vec<(isize, isize)>, start: (usize, usize)) -> bool {
    if field[start.0][start.1] != '.' {
        return false;
    }
    let (mut y, mut x) = (start.0 as isize, start.1 as isize);
    for (dy, dx) in moves {
        y = y as isize + dy;
        x = x as isize + dx;
        if field[y as usize][x as usize] != '.' {
            return false;
        }
    }
    true
}
