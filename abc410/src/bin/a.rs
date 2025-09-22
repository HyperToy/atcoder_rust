use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        k: i32,
    }
    println!("{}", a.iter().filter(|&&x| k <= x).count());
}
