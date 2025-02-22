use proconio::input;

fn main() {
    input! {
        x: i32, y: i32, z: i32,
    }
    println!("{}", (x - z) / (y + z));
}
