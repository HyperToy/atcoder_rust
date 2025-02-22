use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
        qs: [(usize, Usize1, Usize1); q],
    }

    let mut rows: Vec<_> = (0..n).collect();
    for (t, x, y) in qs {
        match t {
            1 => {
                let tmp = rows[x];
                rows[x] = rows[y];
                rows[y] = tmp;
            }
            2 => {
                println!("{}", a[rows[x]][y]);
            }
            _ => unreachable!(),
        }
    }
}
