use ac_library::ModInt998244353 as Mint;
use proconio::input;
fn main() {
    input! {
        n: u64, m: u64,
    }
    let mut after = 0;
    let mut answer = Mint::from(0);
    for i in 0..60 {
        let mut now = Mint::from(0);
        now += (n >> (i + 1)) * (1 << i);
        now += if (n >> i) & 1 == 1 { after + 1 } else { 0 };
        if (m >> i) & 1 == 1 {
            answer += now;
        }
        after += n & (1 << i);
    }
    println!("{}", answer);
}
