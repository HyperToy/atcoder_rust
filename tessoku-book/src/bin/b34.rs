use proconio::*;

fn main() {
    input! {
        n: usize, _x: usize, _y: usize,
        a: [usize; n],
    }
    let mut xor_sum = 0;
    for i in 0..n {
        xor_sum ^= match a[i] % 5 {
            0 | 1 => 0,
            2 | 3 => 1,
            4 => 2,
            _ => unreachable!(),
        }
    }
    println!("{}", if xor_sum == 0 { "Second" } else { "First" });
}
