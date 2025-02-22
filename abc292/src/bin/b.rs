use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        queries: [(usize, Usize1); q],
    }
    let mut status = vec![0; n];
    for (c, x) in queries {
        match c {
            1 => {
                status[x] += 1;
            }
            2 => {
                status[x] += 2;
            }
            3 => {
                println!(
                    "{}",
                    match status[x] {
                        2 | 3 => "Yes",
                        _ => "No",
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
