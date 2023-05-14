use proconio::*;

fn main() {
    input! {
        s: String,
    }
    let ans: usize = s.chars().filter(|c| *c == '1').count();
    println!("{}", ans);
}
