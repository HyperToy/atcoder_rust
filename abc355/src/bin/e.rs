use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize, (l, r): (u64, u64),
    }
    let r = r + 1;
    let mut queries = make_query(l, r);
    let mut left = vec![];
    let mut right = vec![];

    let (li, lj) = queries[0];
    let (ri, rj) = queries[queries.len() - 1];

    for a in 0..lj {
        if lj >> (a - 1) == 0 {
            break;
        }
        for b in 0..rj {
            if rj >> (b - 1) == 0 {
                break;
            }
            let ll = make_l(li + a, lj >> a);
            let rr = make_r(ri + b, rj >> b);
            let now_query = make_query(ll, rr);
            let now_left = make_query(ll, l);
            let now_right = make_query(r, rr);
            if now_query.len() + now_left.len() + now_right.len()
                < queries.len() + left.len() + right.len()
            {
                queries = now_query;
                left = now_left;
                right = now_right
            }
        }
    }

    let mut answer = 0;
    for (l, r) in queries {
        println!("? {} {}", l, r);
        stdout().flush().unwrap();
        input! {
            from &mut source,
            t: i32,
        }
        if t == -1 {
            return;
        }
        answer += t;
        answer %= 100;
    }
    for (l, r) in left {
        println!("? {} {}", l, r);
        stdout().flush().unwrap();
        input! {
            from &mut source,
            t: i32,
        }
        if t == -1 {
            return;
        }
        answer += 100 - t;
        answer %= 100;
    }
    for (l, r) in right {
        println!("? {} {}", l, r);
        stdout().flush().unwrap();
        input! {
            from &mut source,
            t: i32,
        }
        if t == -1 {
            return;
        }
        answer += 100 - t;
        answer %= 100;
    }

    println!("! {}", answer);
}

fn make_query(mut l: u64, r: u64) -> Vec<(u64, u64)> {
    let mut queries = vec![];
    while l != r {
        let mut i = 0;
        while l % (1 << i + 1) == 0 && l + (1 << i + 1) <= r {
            i += 1
        }
        queries.push((i, l / (1 << i)));
        l += 1 << i;
    }
    queries
}

fn make_l(i: u64, j: u64) -> u64 {
    (1 << i) * j
}
fn make_r(i: u64, j: u64) -> u64 {
    (1 << i) * (j + 1)
}
