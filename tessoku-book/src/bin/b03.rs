use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut count = 0;
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                if a[i] + a[j] + a[k] == 1000 {
                    count += 1;
                }
            }
        }
    }
    println!("{}", if count > 0 { "Yes" } else { "No" });
}
