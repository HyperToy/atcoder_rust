use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    let mut answer = -1;
    for i in 1..n {
        if a[i - 1] + 1 != a[i] {
            answer = a[i - 1] + 1;
        }
    }
    println!("{}", answer);
}
