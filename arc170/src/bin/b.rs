use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut bad_count = 0;
    let m = 20; // 「悪い連続部分列」の長さの最大値
    for l in 0..n {
        let mut ns = HashSet::new();
        let mut want = HashSet::new();
        for r in l..n.min(l + m) {
            if want.contains(&a[r]) {
                break;
            }
            bad_count += 1;
            for &p in &ns {
                want.insert(2 * a[r] - p);
            }
            ns.insert(a[r]);
        }
    }
    println!("{}", n * (n + 1) / 2 - bad_count);
}
