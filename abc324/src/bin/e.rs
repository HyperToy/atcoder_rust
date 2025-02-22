use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, t: Chars,
        s: [Chars; n],
    }
    let l = t.len();
    // count[i] := t を i 文字目 (0-indexed) の直前で前後に分けたとき、
    //             (前, 後) を部分文字列として含む s[j] の個数
    let mut count = vec![(0, 0); l + 1];
    for s in s {
        let m = s.len();
        let mut i = 0;
        let mut j = 0;
        while i < l && j < m {
            if t[i] == s[j] {
                i += 1;
            }
            j += 1;
        }
        count[i].0 += 1;
        let mut i = l;
        let mut j = m;
        while i > 0 && j > 0 {
            if t[i - 1] == s[j - 1] {
                i -= 1;
            }
            j -= 1;
        }
        count[i].1 += 1;
    }
    let mut sum = 0;
    let mut answer = 0i64;
    for i in 0..=l {
        sum += count[i].1;
        answer += count[i].0 * sum;
    }
    println!("{}", answer);
}
