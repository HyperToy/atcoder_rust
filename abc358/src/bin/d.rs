use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        b: [i64; m],
    }
    let a = a.into_iter().sorted().collect_vec();
    let b = b.into_iter().sorted().collect_vec();
    let mut i = 0;
    let mut j = 0;
    let mut answer = 0i64;
    while i < n && j < m {
        if a[i] >= b[j] {
            answer += a[i];
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }
    println!("{}", if j == m { answer } else { -1 });
}
