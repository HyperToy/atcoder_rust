use itertools::Itertools;
use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        _n: usize, q: usize,
        s: Chars,
        qs: [(Usize1, char); q],
    }
    let mut count = judge_abc_count(&s);
    let mut s = s;
    let mut answer = vec![];
    for (x, c) in qs {
        count = process_query(&mut s, (x, c), count);
        answer.push(count);
    }
    println!("{}", answer.iter().join("\n"));
}

fn process_query(s: &mut [char], query: (usize, char), count: usize) -> usize {
    let (x, c) = query;
    let min = if x < 2 { 0 } else { x - 2 };
    let max = if x + 3 > s.len() { s.len() } else { x + 3 };
    let count = count - judge_abc_count(&s[min..max]);
    s[x] = c;
    let count = count + judge_abc_count(&s[min..max]);
    count
}

fn judge_abc_count(s: &[char]) -> usize {
    s.iter()
        .tuple_windows()
        .filter(|(&a, &b, &c)| judge_abc((a, b, c)))
        .count()
}

fn judge_abc(s: (char, char, char)) -> bool {
    s == ('A', 'B', 'C')
}
