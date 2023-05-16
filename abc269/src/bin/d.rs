use proconio::*;
use std::collections::HashMap;
use std::collections::VecDeque;

const DX: [i32; 6] = [-1, -1, 0, 0, 1, 1];
const DY: [i32; 6] = [-1, 0, -1, 1, 0, 1];
// TODO: なぜコンパイルが通ったかわからない。
fn main() {
    input! {
        n:usize,
        ps:[(i32, i32); n],
    }
    let mut ans = 0;
    let mut map = HashMap::new();
    let mut seen = HashMap::new();
    for i in 0..n {
        map.insert(ps[i].clone(), 1);
        seen.insert(ps[i].clone(), false);
    }
    for p in ps {
        if Some(&true) == seen.get(&p) {
            continue;
        }
        ans += 1;
        let mut v = VecDeque::new();
        v.push_back(p);
        while !v.is_empty() {
            let p = v.pop_front().unwrap();
            for k in 0..6 {
                let tp = (p.0 + DX[k], p.1 + DY[k]);
                if None == map.get(&tp) {
                    continue;
                }
                match seen.get(&tp) {
                    Some(x) => {
                        if *x {
                            continue;
                        }
                    }
                    None => (),
                }
                seen.insert(tp, true);
                v.push_back(tp);
            }
        }
    }
    println!("{}", ans);
}
