use proconio::{input, source::line::LineSource};
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

// interactive
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize, (l, r): (usize, usize),
    }
    let n2 = 1 << n;
    let r = r + 1;
    let mut dist = vec![std::i32::MAX; n2 + 1];
    let mut pre = vec![std::usize::MAX; n2 + 1];
    let mut q = VecDeque::new();
    q.push_back(l);
    dist[l] = 0;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        let mut push = |to| {
            if dist[to] < dist[u] + 1 {
                return;
            }
            dist[to] = dist[u] + 1;
            pre[to] = u;
            q.push_back(to);
        };
        for i in 0..=n {
            if u >= 1 << i {
                push(u - (1 << i));
            }
            if u + (1 << i) <= n2 {
                push(u + (1 << i));
            }
            if u >> i & 1 == 1 {
                break;
            }
        }
    }
    let mut answer = 0;
    let mut pos = r;
    while pos != l {
        let t = query(pre[pos], pos, &mut source);
        answer = (answer + t + 100) % 100;
        pos = pre[pos];
    }
    println!("! {}", answer);
}

fn query<R: BufRead>(l: usize, r: usize, source: &mut LineSource<R>) -> i32 {
    let (l, r, sign) = if l < r { (l, r, 1) } else { (r, l, -1) };
    let i = (r - l).trailing_zeros();
    let j = l / (r - l);
    println!("? {} {}", i, j);
    stdout().flush().unwrap();
    input! {
        from source,
        t: i32,
    }
    t * sign
}
