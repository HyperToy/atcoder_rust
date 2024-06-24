use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
    }
    let mut es = Vec::new();
    for _ in 0..m {
        input! {
            k: usize, c: i64,
            a: [Usize1; k],
        }
        es.push((c, a));
    }
    es.sort();
    let mut dsu = Dsu::new(n);
    let mut answer = 0;
    for (c, a) in es {
        for i in 1..a.len() {
            if !dsu.same(a[0], a[i]) {
                dsu.merge(a[0], a[i]);
                answer += c;
            }
        }
    }
    println!("{}", if dsu.size(0) == n { answer } else { -1 });
}
