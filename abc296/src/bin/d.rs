use proconio::input;

fn main() {
    input! {
        n: i64, m: i64,
    }
    let mut x = None;
    for a in 1..=n {
        let b = (m + a - 1) / a;
        if a > b {
            break;
        }
        if b <= n {
            x = Some(x.map_or(a * b, |x| (a * b).min(x)));
        }
    }
    println!("{}", x.unwrap_or(-1));
}
