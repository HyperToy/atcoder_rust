use proconio::*;

fn diff(a: f32, b: f32) -> f32 {
    (a - b).abs()
}

fn main() {
    input! {
        n: usize,
        t: f32, a: f32,
        h: [f32;n],
    }

    let mut ans = 0;
    let mut mindiff: f32 = 10000.0;
    for i in 0..n {
        let nowdiff = diff(a, t - h[i] * 0.006);
        if mindiff > nowdiff {
            mindiff = nowdiff;
            ans = i + 1;
        }
    }

    println!("{}", ans);
}
