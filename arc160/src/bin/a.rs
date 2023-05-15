use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [i32; n],
    }
    let mut l = 1;
    let mut r = n * (n + 1) / 2;
    for i in 0..n {
        let mut small = Vec::new();
        let mut large = Vec::new();
        for j in i + 1..n {
            if a[j] < a[i] {
                small.push(a[j]);
            }
            if a[j] > a[i] {
                large.push(a[j]);
            }
        }
        let mut x = None;
        if k - l < small.len() {
            small.sort();
            x = Some(small[k - l]);
        }
        if r - k < large.len() {
            large.sort();
            large.reverse();
            x = Some(large[r - k]);
        }
        if let Some(x) = x {
            let mut j = i;
            while a[j] != x {
                j += 1;
            }
            a[i..j + 1].reverse();
            break;
        }
        l += small.len();
        r -= large.len();
    }
    println!("{}", a.iter().join(" "));
}
