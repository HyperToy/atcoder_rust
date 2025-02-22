use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }
    s.sort();
    s.push(String::new());
    let mut stack = Vec::new();
    let mut answer = 0;
    for (i, s) in s.iter().enumerate() {
        let lcp = stack
            .iter()
            .zip(s.chars())
            .take_while(|&(&(c, _), d)| c == d)
            .count();
        while stack.len() > lcp {
            let (_, start) = stack.pop().unwrap();
            answer += (i - start) * (i - start - 1) / 2;
        }
        for c in s[lcp..].chars() {
            stack.push((c, i));
        }
    }
    println!("{}", answer);
}
