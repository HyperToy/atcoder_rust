use proconio::*;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
        qs: [usize; q],
    }
    c.sort();
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = c[i] + sum[i];
    }
    for x in qs {
        let mut ok = 0;
        let mut ng = n + 1;
        while ok + 1 < ng {
            let wj = (ok + ng) / 2;
            if sum[wj] <= x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        println!("{}", ok);
    }
}
