use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans: i64 = 0;
    if a[0] != 0 {
        ans = -1;
    }
    for i in 1..n {
        if a[i] > a[i - 1] + 1 {
            ans = -1;
        }
        if ans == -1 {
            break;
        }
        ans += if a[i] == a[i - 1] + 1 {
            1
        } else {
            a[i]
        }
    }
    println!("{}", ans);
}
