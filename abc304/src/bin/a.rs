use proconio::*;

fn main() {
    input! {
        n: usize,
        members: [(String, i32); n],
    }
    let mut min = 1_000_000_001;
    let mut idx = 0;
    for i in 0..n {
        if members[i].1 < min {
            min = members[i].1;
            idx = i;
        }
    }
    for _ in 0..n {
        println!("{}", members[idx % n].0);
        idx += 1;
    }
}
