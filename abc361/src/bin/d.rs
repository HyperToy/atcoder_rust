use proconio::{input, marker::Chars};
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let n = n + 2;
    let s = f(s, n);
    let t = f(t, n);
    let s = enc(s, n);
    let t = enc(t, n);
    let mut q = VecDeque::new();
    let mut dist = HashMap::new();
    q.push_back(s);
    dist.insert(s, 0);

    while let Some(u) = q.pop_front() {
        let now_d = *dist.get(&u).unwrap();
        for v in neighbor(u, n) {
            if dist.get(&v).is_some_and(|&d| d <= now_d + 1) {
                continue;
            }
            dist.insert(v, now_d + 1);
            q.push_back(v);
        }
    }
    println!("{}", dist.get(&t).unwrap_or(&(-1)));
}
fn f(s: Vec<char>, n: usize) -> Vec<CellStatus> {
    let mut res = vec![CellStatus::Empty; n];
    for (i, c) in s.into_iter().enumerate() {
        res[i] = match c {
            'B' => CellStatus::Black,
            'W' => CellStatus::White,
            _ => unreachable!(),
        };
    }
    res
}
fn enc(s: Vec<CellStatus>, n: usize) -> i32 {
    let mut res = 0;
    for i in 0..n {
        res *= 3;
        if i < n {
            res += match s[i] {
                CellStatus::Black => 2,
                CellStatus::White => 1,
                CellStatus::Empty => 0,
            }
        }
    }
    res
}
fn dec(mut s: i32, n: usize) -> Vec<CellStatus> {
    let mut res = vec![CellStatus::Empty; n];
    for i in (0..n).rev() {
        res[i] = match s % 3 {
            2 => CellStatus::Black,
            1 => CellStatus::White,
            0 => CellStatus::Empty,
            _ => unreachable!(),
        };
        s /= 3;
    }
    res
}
fn neighbor(s: i32, n: usize) -> Vec<i32> {
    let mut res = Vec::new();
    let s = dec(s, n);
    let k = where_is_empty(&s, n).unwrap();
    for i in 0..n - 1 {
        if s[i] != CellStatus::Empty && s[i + 1] != CellStatus::Empty {
            let mut t = s.clone();
            t[k] = t[i];
            t[k + 1] = t[i + 1];
            t[i] = CellStatus::Empty;
            t[i + 1] = CellStatus::Empty;
            res.push(enc(t, n));
        }
    }
    res
}
fn where_is_empty(s: &Vec<CellStatus>, n: usize) -> Option<usize> {
    for i in 0..n - 1 {
        if s[i] == CellStatus::Empty && s[i + 1] == CellStatus::Empty {
            return Some(i);
        }
    }
    None
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum CellStatus {
    Black,
    White,
    Empty,
}
