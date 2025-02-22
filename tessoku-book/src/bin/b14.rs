use proconio::*;

fn main() {
    input! {
        n: usize, k: i32,
        a: [i32; n],
    }
    let mut p = Vec::new();
    let mut q = Vec::new();

    for i in 0..n {
        if i % 2 == 0 {
            p.push(a[i]);
        } else {
            q.push(a[i]);
        }
    }

    let make = |v: Vec<i32>| {
        let mut res = Vec::new();
        for mask in 0..(1 << v.len()) {
            let mut sum = 0;
            for i in 0..v.len() {
                if (mask >> i) & 1 == 1 {
                    sum += v[i];
                }
            }
            res.push(sum);
        }
        res.sort();
        res
    };
    let p = make(p);
    let q = make(q);

    let exists = |x: i32, v: &Vec<i32>| {
        let mut ok = 0;
        let mut ng = v.len();
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
    for i in 0..p.len() {
        ok |= exists(k - p[i], &q);
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
