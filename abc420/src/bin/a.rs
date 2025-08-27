use proconio::input;

fn main() {
    input! {
        x: i32, y: i32,
    }
    println!("{}", (x + y - 1) % 12 + 1);
}
