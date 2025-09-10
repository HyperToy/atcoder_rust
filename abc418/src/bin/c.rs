use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        qs: [usize; q],
    }
    let (s, max) = f(a);
    let answer = qs
        .into_iter()
        .map(|q| if q > max { -1 } else { s[q] })
        .join("\n");
    println!("{}", answer);
}

fn f(a: Vec<usize>) -> (Vec<i64>, usize) {
    let max = *a.iter().max().unwrap();
    let mut s = vec![0; max + 1];
    for a in a {
        s[a] += 1;
    }
    for i in (1..max).rev() {
        s[i] += s[i + 1];
    }
    for i in 0..max {
        s[i + 1] += s[i];
    }
    for i in (0..max).rev() {
        s[i + 1] = s[i] + 1;
    }
    (s, max)
}
