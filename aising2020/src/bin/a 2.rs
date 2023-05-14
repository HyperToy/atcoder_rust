use proconio::*;

fn main() {
    input! {
        l: u32, r: u32,
        d: u32,
    }
    let mut ans: u32 = 0;
    for i in l..=r {
        if i % d == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
