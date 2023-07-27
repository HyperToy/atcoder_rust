use proconio::*;

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
        if check(wj, &a) >= k + 1 {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{}", ok);
}

fn check(x: usize, a: &Vec<usize>) -> usize {
    let mut result = 0;
    let mut pos = 0;
    for i in 0..a.len() {
        if a[i] - pos >= x {
            result += 1;
            pos = a[i];
        }
    }
    result
}
