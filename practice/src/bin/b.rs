use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::{
    cmp::Ordering,
    io::{stdin, BufRead, BufReader},
};

// merge sort
//  todo (N, Q) = (5, 7) のケースで足りない EBDAC など
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize, mut q: usize,
    }
    let mut v = (0..n).collect::<Vec<_>>();
    let mut memo = vec![vec![None; n]; n];
    merge_sort(&mut v, 0, n, &mut memo, &mut q, &mut source);
    eprintln!("{:?}", memo);
    answer(v);
}

fn merge_sort<R: BufRead>(
    v: &mut Vec<usize>,
    left: usize,
    right: usize,
    memo: &mut Vec<Vec<Option<Ordering>>>,
    q: &mut usize,
    source: &mut LineSource<R>,
) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(v, left, mid, memo, q, source);
        merge_sort(v, mid, right, memo, q, source);
        merge(v, left, mid, right, memo, q, source);
    }
}

fn merge<R: BufRead>(
    v: &mut Vec<usize>,
    left: usize,
    mid: usize,
    right: usize,
    memo: &mut Vec<Vec<Option<Ordering>>>,
    q: &mut usize,
    source: &mut LineSource<R>,
) {
    let nl = mid - left;
    let nr = right - mid;
    let mut l = vec![std::usize::MAX; nl + 1];
    let mut r = vec![std::usize::MAX; nr + 1];
    for i in 0..nl {
        l[i] = v[left + i]
    }
    for i in 0..nr {
        r[i] = v[mid + i];
    }
    let mut j = 0;
    let mut k = 0;
    for i in left..right {
        match query(l[j], r[k], memo, q, source) {
            Ordering::Less | Ordering::Equal => {
                v[i] = l[j];
                j += 1;
            }
            Ordering::Greater => {
                v[i] = r[k];
                k += 1;
            }
        }
    }
}

fn query<R: BufRead>(
    a: usize,
    b: usize,
    memo: &mut Vec<Vec<Option<Ordering>>>,
    q: &mut usize,
    source: &mut LineSource<R>,
) -> Ordering {
    if a == std::usize::MAX {
        return Ordering::Greater;
    }
    if b == std::usize::MAX {
        return Ordering::Less;
    }
    if let Some(o) = memo[a][b] {
        return o;
    }
    assert!(*q > 0);
    *q -= 1;
    println!(
        "? {} {}",
        (a as u8 + b'A') as char,
        (b as u8 + b'A') as char
    );
    input! {
        from source,
        s: char,
    }
    let res = match s {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => unreachable!(),
    };
    memo[a][b] = Some(res);
    memo[b][a] = Some(rev(res));
    generalize(memo);
    res
}

fn generalize(memo: &mut Vec<Vec<Option<Ordering>>>) {
    let n = memo.len();
    eprintln!("{:?}", memo);
    for a in 0..n {
        for b in 0..n {
            for i in 0..n {
                if let Some(x) = memo[a][i] {
                    if let Some(y) = memo[i][b] {
                        if x == y {
                            memo[a][b] = Some(x);
                            memo[b][a] = Some(rev(x));
                        }
                    }
                }
            }
        }
    }
    eprintln!("{:?}", memo);
}
fn rev(o: Ordering) -> Ordering {
    match o {
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
    }
}
fn answer(v: Vec<usize>) {
    println!("! {}", v.iter().map(|x| (*x as u8 + b'A') as char).join(""));
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        assert!(std::usize::MAX == std::usize::MAX);
    }
}
