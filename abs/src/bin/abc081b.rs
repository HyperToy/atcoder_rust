use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a:[i32;n],
    }
    let mut ans: i32 = 0;
    loop {
        let mut ok: bool = true;
        for i in 0..n {
            if a[i] % 2 == 1 {
                ok = false;
            } else {
                a[i] /= 2;
            }
        }
        if !ok {
            break;
        }
        ans += 1;
    }
    println! ("{}", ans);
}
