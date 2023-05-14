use proconio::*;

fn main() {
    input! {
        s: String,
    }
    let l: usize = 12;
    let mut ans: u32 = 0;
    for i in 0..l-3 {
        if &s[i..i+4] == "ZONe" {
            ans += 1;
        }
    }
    println!("{}", ans);
}
