use itertools::Itertools;
use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize, k: usize,
        p: [usize; n],
    }
    let mut l = n - k;
    while l >= 1 && p[l - 1] < p[l] {
        l -= 1;
    }
    let a = p
        .iter()
        .take(n - k)
        .cloned()
        .chain(p.iter().skip(n - k).cloned().sorted())
        .collect_vec();
    let b = p
        .iter()
        .take(l)
        .cloned()
        .chain(p.iter().skip(l).take(k).cloned().sorted())
        .chain(p.iter().skip(l + k).cloned())
        .collect_vec();
    let mut answer = a;
    if cmp(&answer, &b) == Ordering::Less {
        answer = b;
    }
    if sliding_max(&p) >= k && cmp(&answer, &p) == Ordering::Less {
        answer = p;
    }
    println!("{}", answer.iter().join(" "));
}
fn sliding_max<T: Ord + Copy>(a: &Vec<T>) -> usize {
    let mut q = Vec::new();
    let n = a.len();
    let mut res = 0;
    for i in 0..n {
        if q.last().is_some_and(|&x| x > a[i]) {
            q.clear();
        }
        q.push(a[i]);
        res = res.max(q.len());
    }
    res
}

fn cmp<T: Ord + Copy>(a: &Vec<T>, b: &Vec<T>) -> Ordering {
    for i in 0..a.len().min(b.len()) {
        if i >= a.len() {
            return Ordering::Less;
        }
        if i >= b.len() {
            return Ordering::Greater;
        }
        if a[i] == b[i] {
            continue;
        }
        return a[i].cmp(&b[i]);
    }
    return Ordering::Equal;
}
