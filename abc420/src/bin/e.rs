use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
    }
    let qs = (0..q).map(|_| {
        input! {
            t: u32,
        }
        match t {
            1 => {
                input! {
                    u: Usize1, v: Usize1,
                }
                (t, u, v)
            }
            2 | 3 => {
                input! {
                    v: Usize1,
                }
                (t, v, 0)
            }
            _ => unreachable!(),
        }
    });

    let mut dsu = Dsu::new(n);
    let mut black = vec![false; n];
    let mut count = vec![0; n];
    let answer = qs.filter_map(|q| match q {
        (1, u, v) => {
            if !dsu.same(u, v) {
                let c = count[dsu.leader(u)] + count[dsu.leader(v)];
                dsu.merge(u, v);
                count[dsu.leader(u)] = c;
            }
            None
        }
        (2, v, _) => {
            count[dsu.leader(v)] += if black[v] { -1 } else { 1 };
            black[v] ^= true;
            None
        }
        (3, v, _) => Some(count[dsu.leader(v)] > 0),
        _ => unreachable!(),
    });

    println!("{}", answer.map(yes_no).join("\n"))
}

fn yes_no(b: bool) -> String {
    if b { "Yes" } else { "No" }.to_string()
}
