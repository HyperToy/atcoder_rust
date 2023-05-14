use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    for i in 0..n {
        if i == n - 1 {
            print!("{} ", a[i]);
        }else {
            print!("{} ", a[i]);
            if (a[i] - a[i+1]).abs() == 1 {
                continue;
            }
            if a[i] < a[i+1] {
                for j in a[i] + 1 .. a[i+1] {
                    print!("{} ", j);
                }
            } else {
                for j in (a[i + 1]+1..a[i]).rev() {
                    print!("{} ", j);
                }
            }
        }
    }
}
