use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        events: [(u8, Usize1); n],
    }
    let mut r_answers: Vec<i32> = Vec::new();
    let mut k = 0;
    let mut max_k = 0;
    let mut monsters = vec![0; n];
    for (t, x) in events.into_iter().rev() {
        match t {
            1 => {
                if monsters[x] > 0 {
                    r_answers.push(1);
                    k -= 1;
                    monsters[x] -= 1;
                } else {
                    r_answers.push(0);
                }
            }
            2 => {
                monsters[x] += 1;
                k += 1;
                max_k = max_k.max(k);
            }
            _ => unreachable!(),
        }
    }
    let ok = monsters.iter().all(|x| *x == 0);
    println!("{}", if ok { max_k } else { -1 });
    if ok {
        println!("{}", r_answers.iter().rev().join(" "));
    }
}
