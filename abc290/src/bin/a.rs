use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        b: [Usize1; m],
    }
    println!("{}", b.into_iter().map(|i| a[i]).sum::<i32>());
}
