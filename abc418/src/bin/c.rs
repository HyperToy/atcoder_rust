use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        qs: [usize; q],
    }
    let (s, max) = f(&a);
    let answer = qs
        .into_iter()
        .map(|q| if q > max { None } else { s[q] })
        .map(to_string_or_minus_one)
        .join("\n");
    println!("{}", answer);
}

fn f(a: &Vec<usize>) -> (Vec<Option<usize>>, usize) {
    let n = a.len();
    let max = *a.iter().max().unwrap();
    let mut sum = vec![0; max + 1];
    let mut count = vec![0; max + 1];
    for a in a {
        sum[*a] += *a;
        count[*a] += 1;
    }
    for i in 1..=max {
        sum[i] += sum[i - 1];
        count[i] += count[i - 1];
    }
    let answer = std::iter::once(None)
        .chain(
            (1..=max)
                .map(|b| sum[b - 1] + (n - count[b - 1]) * (b - 1) + 1)
                .map(|x| Some(x)),
        )
        .collect_vec();
    (answer, max)
}

fn to_string_or_minus_one(x: Option<usize>) -> String {
    match x {
        Some(x) => x.to_string(),
        None => "-1".to_string(),
    }
}
