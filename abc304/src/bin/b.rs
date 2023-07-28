use proconio::*;

fn main() {
    input! {
        mut n: usize,
    }
    let mut pow10 = vec![1; 10];
    for i in 1..10 {
        pow10[i] = pow10[i - 1] * 10;
    }
    // 最上位は 10^i の位
    let mut i = 0;
    while n >= pow10[i] {
        i += 1;
    }
    if i >= 3 {
        n /= pow10[i - 3];
        n *= pow10[i - 3];
    }
    println!("{}", n);
}
