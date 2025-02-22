use proconio::*;

fn main() {
    input! {
        n: usize, x: i32,
        a: [i32; n],
    }
    // a[i] <= x を満たす最大の i を求める
    let mut ok = 0;
    let mut ng = n;
    while ng - ok > 1 {
        let wj = (ok + ng) / 2;
        if a[wj] <= x {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{}", ok + 1);
}
