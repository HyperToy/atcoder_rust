use proconio::input;

fn main() {
    input! {
        _n: usize, k: usize,
        a: [i64; k],
    }
    println!("{}", if k % 2 == 0 { even(a) } else { odd(a) });
}

fn even(a: Vec<i64>) -> i64 {
    let n = a.len();
    let mut res = 0;
    for i in 0..n {
        res += a[i] * if i % 2 == 0 { -1 } else { 1 }
    }
    res
}
fn odd(a: Vec<i64>) -> i64 {
    let n = a.len();
    let mut sum = vec![0; n];
    let mut rev_sum = vec![0; n];
    for i in 0..n {
        sum[i] = if i > 0 { sum[i - 1] } else { 0 }
            + a[i]
                * match i % 2 {
                    0 => -1,
                    1 => 1,
                    _ => unreachable!(),
                };
        let ri = n - 1 - i;
        rev_sum[ri] = if i > 0 { rev_sum[ri + 1] } else { 0 }
            + a[ri]
                * match i % 2 {
                    0 => 1,
                    1 => -1,
                    _ => unreachable!(),
                };
    }
    let mut res = 1_000_000_000;
    for i in 0..n {
        res = res
            .min(if i > 0 { sum[i - 1] } else { 0 } + if i < n - 1 { rev_sum[i + 1] } else { 0 });
    }
    res
}
