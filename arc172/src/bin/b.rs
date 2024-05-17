use ac_library::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: i64, k: i64, l: i64,
    }
    let mut answer = ModInt998244353::from(1);
    for i in 0..n {
        if i < n - k {
            answer *= 0.max(l - i);
        } else {
            answer *= 0.max(l - n + k);
        }
    }
    println!("{}", answer.val());
}
