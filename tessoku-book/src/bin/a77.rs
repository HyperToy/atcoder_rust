use proconio::*;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, l: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.push(l);
    let mut ok = 1;
    let mut ng = l + 1;
    while ok + 1 < ng {
        let wj = (ok + ng) / 2;
        if check(wj) {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{}", ok);
}
