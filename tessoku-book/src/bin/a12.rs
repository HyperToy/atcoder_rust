/**
 * 答えで二分探索
 * 題意を十分に満たす X のうち、最小のものが求めるべき答え。
 * すなわち、 X は題意を満たすが(ok)、X - 1 は題意を満たさない(ng)。
 * ok = ∞ のとき、題意を満たす。 ng = 0 のとき、題意を満たさない。
 */
use proconio::*;

fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    let check = |wj| {
        let mut sum = 0;
        for x in &a {
            sum += wj / x;
        }
        sum >= k
    };
    let mut ng = 0;
    let mut ok = 1_000_000_000;
    while ok - ng > 1 {
        let wj = (ok + ng) / 2;
        if check(wj) {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{}", ok);
}
