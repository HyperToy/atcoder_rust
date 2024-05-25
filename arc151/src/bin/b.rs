use ac_library::{Dsu, ModInt998244353 as Mint};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: i64,
        p: [Usize1; n],
    }
    let mut dsu = Dsu::new(n);
    let mut answer = Mint::from(0);
    let m = Mint::from(m);
    let mc2 = m * (m - 1) / 2;
    let mut group_count = n as u64;
    for i in 0..n {
        if !dsu.same(i, p[i]) {
            answer += mc2 * m.pow(group_count - 2);
            dsu.merge(i, p[i]);
            group_count -= 1;
        }
    }
    println!("{}", answer);
}
