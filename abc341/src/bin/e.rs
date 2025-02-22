use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
        queries: [(u8, Usize1, Usize1); q],
    }
    let mut set = BTreeSet::new();
    for i in 1..n {
        if s[i - 1] == s[i] {
            set.insert(i);
        }
    }
    let mut answer = vec![];
    for (t, l, r) in queries {
        match t {
            1 => {
                if 0 < l {
                    if set.contains(&l) {
                        set.remove(&l);
                    } else {
                        set.insert(l);
                    }
                }
                if r < n - 1 {
                    if set.contains(&(r + 1)) {
                        set.remove(&(r + 1));
                    } else {
                        set.insert(r + 1);
                    }
                }
            }
            2 => {
                answer.push(match set.range(..=r).next_back() {
                    Some(x) => *x <= l,
                    None => true,
                });
            }
            _ => unreachable!(),
        }
    }
    println!(
        "{}",
        answer
            .iter()
            .map(|b| if *b { "Yes" } else { "No" })
            .join("\n")
    );
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    #[test]
    fn learn() {
        let mut set = BTreeSet::new();
        set.insert(1);
        set.insert(3);
        set.insert(5);
        set.insert(7);

        // x 未満の要素の中で最大のものを取得
        assert_eq!(5, *set.range(..6).next_back().unwrap());
        assert_eq!(3, *set.range(..5).next_back().unwrap());
        // x 以下の要素の中で最大のものを取得
        assert_eq!(5, *set.range(..=6).next_back().unwrap());
        assert_eq!(5, *set.range(..=5).next_back().unwrap());
    }
}
