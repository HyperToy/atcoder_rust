use proconio::input;

fn main() {
    input! {
        n: usize, a: i32, b: i32,
        c: [i32; n],
    }
    println!(
        "{}",
        c.iter().enumerate().find(|(_, &x)| x == a + b).unwrap().0 + 1
    );
}
