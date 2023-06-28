/**
 * 二分探索
 *  A から選ぶプレゼントを1つ決め打つ (A[i] とする)
 *      B から、A[i] + D 以下で最大のものを二分探索
 *      これが A[i] - D 以上であれば採用
 */
use proconio::*;
fn main() {
    input! {
        n: usize, m: usize, d: i64,
        a: [i64; n], mut b: [i64; m],
    }
    b.sort();
    let search = |x| {
        let mut ok = 0;
        let mut ng = m;
        while (ng - ok) > 1 {
            let wj = (ok + ng) / 2;
            if b[wj] <= x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        ok
    };
    let mut answer = -1;
    for i in 0..n {
        let j = search(a[i] + d);
        if (a[i] - b[j]).abs() <= d {
            answer = answer.max(a[i] + b[j]);
        }
    }
    println!("{}", answer);
}
