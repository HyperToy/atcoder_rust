use proconio::*;
use std::collections::HashMap;
use std::collections::VecDeque;

const DX: [i32; 6] = [-1, -1, 0, 0, 1, 1];
const DY: [i32; 6] = [-1, 0, -1, 1, 0, 1];

fn main() {
    input! {
        n: usize,
        ps: [(i32, i32); n],
    }
    let mut map = HashMap::new();
    let mut seen = HashMap::new();
    /*
     * MEMO: イテレータとタプルのコピーについて
     *
     * tuple のそれぞれの型が Copy トレイトを実装しているとき、
     * その tuple 自身も Copy トレイトを実装していることになる。
     *  → ムーブが起こらずコピーされる。
     *
     * arr: Vec<T> として、 for x in arr の形でループするとき、
     * arr に対して暗黙的に (implicitly) .into_iter() が呼ばれている。
     * これは値のムーブを起こしているので、その後参照できない。
     * .into_iter() : ムーブ
     * .iter_mut()  : 可変借用
     * .iter()      : 不変借用
     *
     * イテレータに対してコピーしながら処理を行いたいときは、 .copied() をつける。
     * .iter().copied() : コピー
     */
    // for i in 0..n {
    //     map.insert(ps[i], 1); // copied
    //     seen.insert(ps[i], false); // copied
    // }
    for p in ps.iter().copied() {
        map.insert(p, 1);
        seen.insert(p, false);
    }
    let mut ans = 0;
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
                if let Some(&true) = seen.get(&tp) {
                    continue;
                }
                seen.insert(tp, true);
                v.push_back(tp);
            }
        }
    }
    println!("{}", ans);
}
