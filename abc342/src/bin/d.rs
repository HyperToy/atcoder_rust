use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let zero = a.iter().filter(|x| **x == 0).count();
    let v = a
        .into_iter()
        .filter(|x| *x != 0)
        .map(|x| odd_factors_product(x))
        .collect::<Vec<_>>();
    let mut map = HashMap::new();
    for s in &v {
        *(map.entry(*s).or_insert(0)) += 1;
    }
    println!(
        "{}",
        if zero > 0 { zero * (zero - 1) / 2 } else { 0 }
            + zero * v.len()
            + map.iter().map(|(_, v)| v * (v - 1) / 2).sum::<usize>()
    );
}

// x の素因数のうち、指数が奇数のものの総積
fn odd_factors_product(x: i32) -> i32 {
    x / max_square_divisor(x)
}

// x の平方数の約数のうち、最大のもの
fn max_square_divisor(x: i32) -> i32 {
    let mut res = 1;
    for i in 1..x {
        if i * i > x {
            break;
        }
        if x % (i * i) == 0 {
            res = i * i;
        }
    }
    res
}
