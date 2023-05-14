use proconio::*;

fn main() {
    input! {
        N: u32,
    }

    let mut ans: str = "No";
    for i in 1..10 {
        if i * i == N {
            ans = "Yes";
        }
    }
    println("{}", ans);
}
