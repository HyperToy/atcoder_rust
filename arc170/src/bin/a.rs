use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }
    println!("{}", solve(&s, &t).map_or_else(|| -1, |v| v as isize));
}
fn solve(s: &Vec<char>, t: &Vec<char>) -> Option<usize> {
    let st = s.iter().zip(t.iter()).collect_vec();
    if st
        .iter()
        .take_while(|&&(_, &c)| c == 'B')
        .any(|&(&c, _)| c == 'A')
    {
        return None;
    }
    if st
        .iter()
        .rev()
        .take_while(|&&(_, &c)| c == 'A')
        .any(|&(&c, _)| c == 'B')
    {
        return None;
    }
    let count = st.iter().fold((0, 0), |(todo, done), &st| match st {
        (&'B', &'A') => (todo + 1, done), // 後続の A->B に合わせて操作
        (&'A', &'B') => (todo - todo.min(1), done + 1), // 操作を実行 (先行の B->A があればそれを消費)
        _ => (todo, done),
    });
    Some(count.0 + count.1)
}
