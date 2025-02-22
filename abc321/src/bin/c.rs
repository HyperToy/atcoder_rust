use proconio::*;

fn main() {
    let mut v = Vec::new();
    for i in 2..(1 << 10) {
        let mut x: u64 = 0;
        for j in (0..10).rev() {
            if (i >> j) & 1 == 1 {
                x *= 10;
                x += j;
            }
        }
        v.push(x);
    }
    v.sort();

    input! {
        k: usize,
    }
    println!("{}", v[k - 1]);
}
