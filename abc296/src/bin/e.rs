use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    // ダブリング
    // 数列a を i -> a[i] の写像と見て、 a の n 個の合成写像を考える。
    // n 個合成すれば、十分に「収束」した状態になっている。
    println!("{}", composite_pow(&a, n).iter().sorted().dedup().count());
}
fn composite_pow(a: &Vec<usize>, mut k: usize) -> Vec<usize> {
    let mut mul = a.clone();
    let mut res = (0..a.len()).collect();
    while k > 0 {
        if k % 2 == 1 {
            res = composite2(&res, &mul);
        }
        mul = composite(&mul);
        k /= 2;
    }
    res
}
fn composite(a: &Vec<usize>) -> Vec<usize> {
    (0..a.len()).map(|i| a[a[i]]).collect()
}
fn composite2(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    (0..a.len()).map(|i| a[b[i]]).collect()
}
