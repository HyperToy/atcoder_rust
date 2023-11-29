use proconio::input;

fn main() {
    input! {
        n: usize, x: i32,
        s: [i32; n],
    }
    println!("{}", s.iter().filter(|s| **s <= x).sum::<i32>());
}
