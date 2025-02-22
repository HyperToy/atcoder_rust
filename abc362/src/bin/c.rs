use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }
    let mut x = vec![0; n];
    let mut total = 0;
    for i in 0..n {
        let (l, r) = lr[i];
        if 0 < l {
            x[i] = l;
            total += l;
        } else if r < 0 {
            x[i] = r;
            total += r;
        }
    }
    for i in 0..n {
        if total == 0 {
            break;
        }
        let (l, r) = lr[i];
        if l <= 0 && 0 <= r {
            if total > 0 {
                x[i] = l.max(-total);
                total += l.max(-total);
            } else {
                x[i] = r.min(-total);
                total += r.min(-total);
            }
        } else {
            if total > 0 {
                if r < 0 {
                    let diff = (l - r).max(-total);
                    x[i] += diff;
                    total += diff;
                }
            } else {
                if 0 < l {
                    let diff = (r - l).min(-total);
                    x[i] += diff;
                    total += diff;
                }
            }
        }
    }
    println!(
        "{}",
        if total == 0 {
            format!("{}\n{}", "Yes", x.into_iter().join(" "))
        } else {
            "No".to_string()
        }
    );
}
