use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        abcd: [(Usize1, char, Usize1, char); m],
    }
    let mut dsu = Dsu::new(n);
    let mut x = 0;
    let mut y = n;
    for (a, _, c, _) in abcd {
        if dsu.same(a, c) {
            y -= 1;
            x += 1;
        } else {
            y -= 1;
            dsu.merge(a, c);
        }
    }
    println!("{} {}", x, y);
}
