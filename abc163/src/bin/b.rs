use proconio::input;

fn main() {
    input! {
        n: i32, m: usize,
        a: [i32; m],
    }
    let total = a.iter().sum::<i32>();
    println!("{}", if total > n { -1 } else { n - total });
}
