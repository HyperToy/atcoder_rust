use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut xor = 0;
    for i in 0..n {
        xor ^= a[i];
    }
    println!("{}", if xor == 0 { "Second" } else { "First" });
}
