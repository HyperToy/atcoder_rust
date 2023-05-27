use proconio::*;

fn main() {
    input! {
        n: i32, k: i32,
    }
    let mut ans = 0;
    for a in 1..=n {
        for b in 1..=n {
            let c = k - a - b;
            if 1 <= c && c <= n {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
