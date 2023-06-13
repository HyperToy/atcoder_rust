use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        lr: [(i64, i64); q],
    }
    let mut sleep = vec![0; n];
    for i in 1..n {
        sleep[i] = sleep[i - 1] + if i % 2 == 0 { a[i] - a[i - 1] } else { 0 };
    }
    let get = |t| {
        // t の直前の 起床or就寝 の時刻
        let mut ok = 0;
        let mut ng = n;
        while ok + 1 < ng {
            let wj = (ok + ng) / 2;
            if a[wj] <= t {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        if ok % 2 == 1 {
            // 直前が就寝
            sleep[ok] + (t - a[ok])
        } else {
            sleep[ok]
        }
    };
    for (l, r) in lr {
        println!("{}", get(r) - get(l));
    }
}
