use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize, m: usize,
        s: Chars,
        t: Chars,
        qs: [(Usize1, Usize1); m],
    }
    let mut imos = vec![0; n + 1];
    for (l, r) in qs {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }
    let answer = (0..n)
        .into_iter()
        .map(|i| if imos[i] % 2 == 0 { s[i] } else { t[i] })
        .join("");
    println!("{}", answer);
}
