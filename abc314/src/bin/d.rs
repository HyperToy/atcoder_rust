use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
        q: usize,
        queries: [(i32, usize, char); q],
    }
    let mut last = q;
    for i in (0..q).rev() {
        if queries[i].0 != 1 {
            last = i;
            break;
        }
    }
    for i in 0..q {
        let (t, x, c) = queries[i];
        match t {
            1 => {
                s[x - 1] = c;
            }
            2 if i == last => {
                s = s.iter().map(|c| c.to_ascii_lowercase()).collect_vec();
            }
            3 if i == last => {
                s = s.iter().map(|c| c.to_ascii_uppercase()).collect_vec();
            }
            _ => (),
        }
    }
    println!("{}", s.iter().join(""));
}
