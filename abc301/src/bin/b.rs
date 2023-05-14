use proconio::*;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut res = vec![a[0]];
    for i in 1..n {
        if a[i - 1] < a[i] {
            for x in a[i - 1] + 1..a[i] {
                res.push(x);
            }
        } else {
            for x in (a[i] + 1..a[i - 1]).rev() {
                res.push(x);
            }
        }
        res.push(a[i]);
    }
    println!("{}", res.iter().join(" "));
}
