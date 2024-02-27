use proconio::input;

fn main() {
    input! {
        (a, m): (i64, i64),
        (l, r): (i64, i64),
    }
    let (l, r) = simplify(l, r, a, m);
    println!("{}", r / m + if l == 0 { 1 } else { 0 });
}

fn simplify(mut l: i64, mut r: i64, a: i64, m: i64) -> (i64, i64) {
    l -= a;
    r -= a;
    if l < 0 {
        let k = (-l + m - 1) / m;
        l += k * m;
        r += k * m;
    } else if l >= m {
        let k = l / m;
        l -= k * m;
        r -= k * m;
    }
    (l, r)
}
