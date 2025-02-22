use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (h, w): (isize, isize), n: usize,
    }
    let mut dir = (-1, 0);
    let (mut i, mut j) = (0, 0);
    let mut field = vec![vec!['.'; w as usize]; h as usize];
    for _ in 0..n {
        if field[i as usize][j as usize] == '.' {
            field[i as usize][j as usize] = '#';
            dir = (dir.1, -dir.0);
        } else {
            field[i as usize][j as usize] = '.';
            dir = (-dir.1, dir.0);
        }
        (i, j) = ((i + dir.0 + h) % h, (j + dir.1 + w) % w);
    }
    println!("{}", field.iter().map(|row| row.iter().join("")).join("\n"));
}
