use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    println!("{}", if n >= 0 { (n + 9) / 10 } else { -(-n / 10) });
}
