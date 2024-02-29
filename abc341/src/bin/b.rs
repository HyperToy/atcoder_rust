use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        st: [(i64, i64); n - 1],
    }

    for i in 0..n - 1 {
        a[i + 1] += a[i] / st[i].0 * st[i].1;
    }
    println!("{}", a[n - 1]);
}
