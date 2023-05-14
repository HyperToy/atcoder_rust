use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut ans: u32 = 0;
    for i in 0..n {
        if i % 2 == 0 && a[i] % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
