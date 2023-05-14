use proconio::*;

fn main() {
    input! {
        n: usize,
        st: [(String, u32); n],
        x: String,
    }
    let mut sleep: bool = false;
    let mut ans: u32 = 0;
    for (s, t) in st {
        if sleep {
            ans += t;
        }
        if x == s {
            sleep = true;
        }
    }
    println!("{}", ans);
}
