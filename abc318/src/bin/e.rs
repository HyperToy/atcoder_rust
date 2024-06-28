use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut answer = 0i64;
    let mut mul = 0;
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for i in 0..n {
        right[a[i]] += 1;
    }
    for i in 0..n {
        right[a[i]] -= 1;
        mul -= left[a[i]];
        if i > 0 {
            left[a[i - 1]] += 1;
            mul += right[a[i - 1]];
        }
        answer += mul - left[a[i]] * right[a[i]];
    }
    println!("{}", answer);
}
