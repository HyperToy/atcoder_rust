use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        q: usize,
        queries: [(i32, usize); q],
    }
    let mut a = Vec::new();
    let mut answer = Vec::new();
    for (t, x) in queries {
        match t {
            1 => {
                a.push(x);
            }
            2 => {
                answer.push(a[a.len() - x]);
            }
            _ => unreachable!(),
        }
    }
    println!("{}", answer.iter().join("\n"));
}
