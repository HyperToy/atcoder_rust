use proconio::input;

fn main() {
    input! {
        n: usize, m: i32,
        h: [i32; n],
    }
    let mut answer = 0;
    let mut m = m;
    for i in 0..n {
        if h[i] <= m {
            answer += 1;
            m -= h[i];
        } else {
            break;
        }
    }
    println!("{}", answer);
}
