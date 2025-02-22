use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut e = Mint::from(0);
    let mut sum_e = Mint::from(0);
    let mut sum_a = Mint::from(0);
    for i in (0..n).rev() {
        sum_e += e;
        sum_a += a[i];
        e = (sum_e + sum_a) / n;
    }
    println!("{}", e);
}
