use proconio::input;

fn main() {
    input! {
        (p, q, r): (i32, i32, i32),
    }
    println!("{}", p + q + r - p.max(q).max(r));
}
