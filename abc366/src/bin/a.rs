use proconio::input;

fn main() {
    input! {
        n: i32, t: i32, a: i32,
    }
    println!("{}", if t.max(a) * 2 > n { "Yes" } else { "No" });
}
