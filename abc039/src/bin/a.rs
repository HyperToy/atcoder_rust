use proconio::*;

fn main() {
    input! {
        a: [u32; 3],
    }
    let mut s: u32 = 0;
    for i in 0..3 {
        s += 2 * a[i] * a[(i + 1) % 3];
    }
    println!("{}", s);
}
