use proconio::*;

fn main() {
    input! {
        n: isize,
        mut a: [i32; n],
        q: isize,
        xs: [i32; q],
    }
    a.sort();
    for x in xs {
        // a[i] < x を満たす最大の i を求める
        let mut ok = -1;
        let mut ng = n;
        while ng - ok > 1 {
            let wj = (ok + ng) / 2;
            if a[wj as usize] < x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        println!("{}", ok + 1);
    }
}
