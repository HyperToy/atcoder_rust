use proconio::input;

fn main() {
    input! {
        n: i32, m: usize,
        a: [i32; m],
    }
    println!("{}", (n - a.iter().sum::<i32>()).max(-1));
}
