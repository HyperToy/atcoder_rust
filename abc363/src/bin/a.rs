use proconio::input;

fn main() {
    input! {
        r: i32,
    }
    println!("{}", (r + 100) / 100 * 100 - r);
}
