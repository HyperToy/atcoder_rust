use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    a.reverse();
    let mut ans: i32 = 0;
    for i in 0..n {
        ans += a[i] * if i % 2 == 0 {
            1
        } else {
            -1
        }
    }
    print!("{}", ans);
}
