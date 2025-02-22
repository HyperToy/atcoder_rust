use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let l = n.to_string().len() as u64;
    let ten_power_l = Mint::from(10).pow(l);
    println!(
        "{}",
        Mint::from(n) * (ten_power_l.pow(n) - 1) / (ten_power_l - 1)
    );
}
