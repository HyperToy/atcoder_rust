use proconio::*;

fn main() {
    input! {
        n: usize, p: i32, q: i32,
        a: [i32; n],
    }
    println!("{}", p.min(q + a.iter().min().unwrap()));
}
