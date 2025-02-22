use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, q: usize,
    }
    let mut reversed = false;
    let mut v: Vec<_> = (1..=n).collect();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: Usize1, y: usize,
                }
                if !reversed {
                    v[x] = y;
                } else {
                    v[n - 1 - x] = y;
                }
            }
            2 => {
                reversed = !reversed;
            }
            3 => {
                input! {
                    x: Usize1,
                }
                println!("{}", if !reversed { v[x] } else { v[n - 1 - x] });
            }
            _ => unreachable!(),
        }
    }
}
