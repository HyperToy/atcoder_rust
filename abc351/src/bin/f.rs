use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::input;

// 平面走査
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut s0 = FenwickTree::new(n, 0); // 個数
    let mut s1 = FenwickTree::new(n, 0); // 総和
    let mut answer = 0;
    for (i, a) in a.into_iter().enumerate().sorted_by_key(|p| p.1).rev() {
        let c = s0.sum(i..);
        let s = s1.sum(i..);
        answer += s - c * a;
        s0.add(i, 1);
        s1.add(i, a);
    }
    println!("{}", answer);
}
