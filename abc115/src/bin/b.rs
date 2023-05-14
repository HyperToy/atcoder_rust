use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    a.sort();
    let mut ans: u32 = 0;
    for i in 0..n {
        ans += a[i];
    }
    ans -= a[n - 1] / 2;
    println!("{}", ans);
}
