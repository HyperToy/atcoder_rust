use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; m],
        x: [[i64; m]; n],
    }
    println!(
        "{}",
        if (0..m).all(|j| (0..n).map(|i| x[i][j]).sum::<i64>() >= a[j]) {
            "Yes"
        } else {
            "No"
        }
    )
}
