use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        q: usize,
    }

    let mut map = HashMap::new();
    let head = 0;
    let tail = std::i32::MAX;
    map.insert(head, (head, a[0]));
    map.insert(tail, (a[n - 1], tail));
    for i in 0..n {
        let pre = if i > 0 { a[i - 1] } else { head };
        let nxt = if i < n - 1 { a[i + 1] } else { tail };
        map.insert(a[i], (pre, nxt));
    }
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                input! { x: i32, y: i32 }
                let pre = *map.get(&x).unwrap();
                let nxt = *map.get(&pre.1).unwrap();
                map.insert(x, (pre.0, y));
                map.insert(y, (x, pre.1));
                map.insert(pre.1, (y, nxt.1));
            }
            2 => {
                input! { x: i32 }
                let pos = *map.get(&x).unwrap();
                let pre = *map.get(&pos.0).unwrap();
                let nxt = *map.get(&pos.1).unwrap();
                map.insert(pos.0, (pre.0, pos.1));
                map.insert(pos.1, (pos.0, nxt.1));
            }
            _ => unreachable!(),
        }
    }
    let mut answer = Vec::new();
    let mut now = head;
    loop {
        now = map.get(&now).unwrap().1;
        if now == tail {
            break;
        }
        answer.push(now);
    }
    println!("{}", answer.iter().join(" "));
}
