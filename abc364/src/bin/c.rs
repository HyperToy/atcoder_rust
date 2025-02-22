use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, x: i64, y: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let mut ca = 0;
    let mut sa = 0;
    let mut cb = 0;
    let mut sb = 0;
    for a in a.iter().sorted().rev() {
        sa += a;
        ca += 1;
        if sa > x {
            break;
        }
    }
    for b in b.iter().sorted().rev() {
        sb += b;
        cb += 1;
        if sb > y {
            break;
        }
    }
    println!("{}", ca.min(cb));
}
