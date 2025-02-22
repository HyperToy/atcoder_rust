use itertools::Itertools;
use proconio::{marker::Chars, *};

const HEIGHT: usize = 9;
const WIDTH: usize = 9;
#[allow(non_upper_case_globals)]
const TaK: [[char; WIDTH]; HEIGHT] = [
    ['#', '#', '#', '.', '?', '?', '?', '?', '?'],
    ['#', '#', '#', '.', '?', '?', '?', '?', '?'],
    ['#', '#', '#', '.', '?', '?', '?', '?', '?'],
    ['.', '.', '.', '.', '?', '?', '?', '?', '?'],
    ['?', '?', '?', '?', '?', '?', '?', '?', '?'],
    ['?', '?', '?', '?', '?', '.', '.', '.', '.'],
    ['?', '?', '?', '?', '?', '.', '#', '#', '#'],
    ['?', '?', '?', '?', '?', '.', '#', '#', '#'],
    ['?', '?', '?', '?', '?', '.', '#', '#', '#'],
];

fn main() {
    input! {
        (n, m): (usize, usize),
        s: [Chars; n],
    }
    println!(
        "{}",
        (0..=n - HEIGHT)
            .cartesian_product(0..=m - WIDTH)
            .filter(|(i, j)| (0..HEIGHT)
                .cartesian_product(0..WIDTH)
                .all(|(y, x)| TaK[y][x] == '?' || s[i + y][j + x] == TaK[y][x]))
            .map(|(i, j)| format!("{} {}", i + 1, j + 1))
            .join("\n")
    );
}
