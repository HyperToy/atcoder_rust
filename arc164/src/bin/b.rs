use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
        c: [i32; n],
    }
    let mut dsu = Dsu::new(n);
    for &(a, b) in &es {
        if c[a] != c[b] {
            dsu.merge(a, b);
        }
    }
    println!(
        "{}",
        if es.iter().any(|&(a, b)| c[a] == c[b] && dsu.same(a, b)) {
            "Yes"
        } else {
            "No"
        }
    );
}
