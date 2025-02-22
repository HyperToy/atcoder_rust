use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
    }
    let v = a.into_iter().sorted().rev().collect::<Vec<_>>();
    let mut plate = vec![0; m];

    let mut pos = 0;
    let mut rev = false;
    for x in v {
        plate[pos] += x;
        if rev {
            if pos > 0 {
                pos -= 1;
            }
        } else {
            pos += 1;
        }
        if pos == m {
            pos = m - 1;
            rev = true;
        }
    }
    println!("{}", plate.iter().map(|x| x * x).sum::<i64>());
}
