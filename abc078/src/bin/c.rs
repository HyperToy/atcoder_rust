use proconio::input;

fn main() {
    input! {
        n: u32, m: u32,
    }
    println!("{}", (1900 * m + 100 * (n - m)) * 2u32.pow(m));
}
