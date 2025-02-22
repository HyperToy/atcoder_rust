use proconio::*;

fn main() {
    input! {
        d: isize,
    }
    let mut answer = d;
    for x in 0..=((d as f64).sqrt().ceil() as isize) {
        let c = x * x - d;
        if c >= 0 {
            answer = answer.min(c);
        } else {
            let y1 = (-c as f64).sqrt().floor() as isize;
            answer = answer.min((c + y1 * y1).abs());
            let y2 = y1 + 1;
            answer = answer.min((c + y2 * y2).abs());
        }
    }
    println!("{}", answer);
}
