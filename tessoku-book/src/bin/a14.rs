use proconio::*;

fn main() {
    input! {
        n: usize, k: i32,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
        d: [i32; n],
    }

    let mut v = Vec::new();
    for i in 0..n {
        for j in 0..n {
            v.push(c[i] + d[j]);
        }
    }
    v.sort();

    let exists = |x| {
        let mut ok = 0;
        let mut ng = n * n;
        while ng - ok > 1 {
            let wj = (ok + ng) / 2;
            if v[wj] <= x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        v[ok] == x
    };

    let mut ok = false;
    for i in 0..n {
        for j in 0..n {
            ok |= exists(k - a[i] - b[j]);
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
