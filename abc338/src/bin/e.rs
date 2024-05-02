use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    }
    let mut v = vec![None; n * 2];
    for (a, b) in ab {
        v[a.max(b)] = Some(a.min(b));
    }
    let mut s: Vec<usize> = Vec::new();
    let mut exist = false;
    for i in 0..n * 2 {
        match v[i] {
            Some(x) => {
                if s.pop().unwrap() != x {
                    exist = true;
                    break;
                }
            }
            None => {
                s.push(i);
            }
        }
    }
    println!("{}", if exist { "Yes" } else { "No" });
}
