use proconio::*;

fn main() {
    input! {
        n: f64,
    }
    let f = |x: f64| x.powf(3.) + x;
    let mut ng = 0.;
    let mut ok = 100.;
    while (f(ok) - n).abs() > 1e-6 {
        let wj = (ng + ok) / 2.;
        if f(wj) > n {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{:.6}", ok);
}
