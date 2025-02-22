use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, k: usize,
        x: [Usize1; n],
        a: [i32; n],
    }
    // ダブリング
    // x を i->x[i] の射として、 x の k個の合成
    let x_k = composite_pow(&x, k);
    println!("{}", (0..n).map(|i| a[x_k[i]]).join(" "));
}
fn composite_pow(a: &Vec<usize>, mut k: usize) -> Vec<usize> {
    let mut mul = a.clone();
    let mut res = (0..a.len()).collect();
    while k > 0 {
        if k % 2 == 1 {
            res = composite(&res, &mul);
        }
        mul = composite(&mul, &mul);
        k /= 2;
    }
    res
}
fn composite(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    (0..a.len()).map(|i| a[b[i]]).collect()
}
