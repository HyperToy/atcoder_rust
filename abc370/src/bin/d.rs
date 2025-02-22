use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        h: usize, w: usize, q: usize,
        queries: [(Usize1, Usize1); q],
    }
    let mut rows = vec![(0..w).collect::<BTreeSet<_>>(); h];
    let mut cols = vec![(0..h).collect::<BTreeSet<_>>(); w];
    for (r, c) in queries {
        if rows[r].contains(&c) {
            rows[r].remove(&c);
            cols[c].remove(&r);
            continue;
        }
        // 上
        if let Some(&y) = cols[c].range(..=r).next_back() {
            rows[y].remove(&c);
            cols[c].remove(&y);
        }
        // 下
        if let Some(&y) = cols[c].range(r..).next() {
            rows[y].remove(&c);
            cols[c].remove(&y);
        }
        // 左
        if let Some(&x) = rows[r].range(..=c).next_back() {
            rows[r].remove(&x);
            cols[x].remove(&r);
        }
        // 右
        if let Some(&x) = rows[r].range(c..).next() {
            rows[r].remove(&x);
            cols[x].remove(&r);
        }
    }
    println!("{}", rows.iter().map(|s| s.len()).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn check_btree_set() {
        let mut set = BTreeSet::new();
        set.insert(3);
        set.insert(5);
        set.insert(8);

        assert_eq!(Some(&5), set.range(4..).next());
        assert_eq!(Some(&5), set.range(5..).next());
        assert_eq!(Some(&5), set.range(..6).next_back());
        assert_eq!(Some(&3), set.range(..5).next_back());
        assert_eq!(Some(&5), set.range(..=5).next_back());
    }
}
