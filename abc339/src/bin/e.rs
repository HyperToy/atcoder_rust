use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize, d: usize,
        a: [usize; n],
    }
    let max = *a.iter().max().unwrap() + 1;
    let mut segtree = Segtree::<Max<usize>>::new(max);
    for a in a {
        let l = if a >= d { a - d } else { 0 };
        let r = if a + d < max { a + d + 1 } else { max };
        segtree.set(a, segtree.prod(l..r) + 1);
    }
    println!("{}", segtree.prod(0..max));
}
