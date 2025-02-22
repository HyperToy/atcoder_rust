use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        a: [i32; n],
        bk: [(i32, usize); q],
    }
    let a = a.into_iter().sorted().collect_vec();
    let mut answer = vec![];
    for (b, k) in bk {
        // [b-x, b+x] に含まれる a の要素が k 個以上あるような最小の x
        let mut ok = std::cmp::max(b - a[0], a[n - 1] - b);
        let mut ng = -1;
        while ng + 1 < ok {
            let wj = (ng + ok) / 2;
            if check(b - wj, b + wj, k, &a) {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        answer.push(ok);
    }
    println!("{}", answer.iter().join("\n"));
}
// [l, r] を満たす a の要素は k 個以上あるか
fn check(l: i32, r: i32, k: usize, a: &Vec<i32>) -> bool {
    a.len() >= k + smaller(l, a) + larger(r, a)
}
fn smaller(x: i32, a: &Vec<i32>) -> usize {
    if x <= a[0] {
        return 0;
    }
    let mut l = 0;
    let mut r = a.len();
    while l + 1 < r {
        let m = (l + r) / 2;
        if a[m] < x {
            l = m;
        } else {
            r = m;
        }
    }
    r
}
fn larger(x: i32, a: &Vec<i32>) -> usize {
    let n = a.len();
    if x < a[0] {
        return n;
    }
    let mut l = 0;
    let mut r = n;
    while l + 1 < r {
        let m = (l + r) / 2;
        if x < a[m] {
            r = m;
        } else {
            l = m;
        }
    }
    n - r
}
