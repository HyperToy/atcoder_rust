use itertools::Itertools;
use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(PartialEq, Eq, PartialOrd)]
enum Event {
    Human(usize),
    Somen((u64, u64)),
}
use std::cmp::Ordering;
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Human(_), Self::Somen(_)) => Ordering::Greater, // ??? refactor
            (Self::Human(u), Self::Human(v)) => u.cmp(&v),
            (Self::Somen(x), Self::Somen(y)) => x.cmp(&y),
            _ => Ordering::Greater,
        }
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        tws: [(u64, u64, u64); m],
    }
    let mut line = (0..n).map(|v| Reverse(v)).collect::<BinaryHeap<_>>();
    let mut events = BinaryHeap::new();
    let mut get = vec![0; n];

    for (t, w, s) in tws {
        events.push((Reverse(t), 0, Event::Somen((w, s))));
    }
    while let Some((t, _, e)) = events.pop() {
        match e {
            Event::Human(u) => {
                // eprintln!("time {}: human {} returns", t.0, u);
                line.push(Reverse(u));
            }
            Event::Somen((w, s)) => {
                // eprintln!("time {}: somen {}, {}", t.0, w, s);
                if let Some(v) = line.pop() {
                    // とる
                    get[v.0] += w;
                    events.push((Reverse(t.0 + s), 1, Event::Human(v.0)));
                    // eprintln!("time {}: human {} will return at {}", t.0, v.0, t.0 + s);
                } else {
                    // 流す
                }
            }
        }
    }
    println!("{}", get.iter().join(" "));
}
