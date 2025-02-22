use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        a: [[i32; w]; h],
    }
    println!("{}", rec(0, 0, h, w, &a, &mut vec![]));
}

fn rec(i: usize, j: usize, h: usize, w: usize, a: &Vec<Vec<i32>>, v: &mut Vec<i32>) -> usize {
    let mut res = 0;
    v.push(a[i][j]);
    if i == h - 1 && j == w - 1 {
        if v.iter().sorted().dedup().count() == h + w - 1 {
            res += 1;
        }
    } else {
        if i + 1 < h {
            res += rec(i + 1, j, h, w, a, v);
        }
        if j + 1 < w {
            res += rec(i, j + 1, h, w, a, v);
        }
    }
    v.pop();
    res
}
