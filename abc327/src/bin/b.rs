use proconio::*;

fn main() {
    input! {
        n: i64,
    }
    let mut ans = -1i64;
    for x in 1..16i64 {
        if x.pow(x as u32) == n {
            ans = x;
        }
    }
    println!("{}", ans);
}
