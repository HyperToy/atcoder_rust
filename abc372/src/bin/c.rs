use itertools::Itertools;
use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        _n: usize, q: usize,
        s: Chars,
        qs: [(Usize1, char); q],
    }
    let count = judge_abc_count(&s);
    let answer = qs
        .iter()
        .fold((s, count, vec![]), |(s, count, mut answer), (x, c)| {
            let (s, count) = process_query(&s, (*x, *c), count);
            answer.push(count);
            (s, count, answer)
        });
    println!("{}", answer.2.iter().join("\n"));
}

fn process_query(s: &[char], query: (usize, char), count: usize) -> (Vec<char>, usize) {
    let (x, c) = query;
    let min = if x < 2 { 0 } else { x - 2 };
    let max = if x + 3 > s.len() { s.len() } else { x + 3 };
    let count = count - judge_abc_count(&s[min..max]);
    let s = [&s[0..x], &[c], &s[x + 1..]].concat();
    let count = count + judge_abc_count(&s[min..max]);
    (s, count)
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
