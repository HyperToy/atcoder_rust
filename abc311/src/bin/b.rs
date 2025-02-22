use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: usize, d: usize,
        s: [Chars; n],
    }
    let mut answer = 0;
    let mut now = 0;
    for j in 0..d {
        let mut selectable = true;
        for i in 0..n {
            selectable &= s[i][j] == 'o';
        }
        if selectable {
            now += 1;
        } else {
            answer = answer.max(now);
            now = 0;
        }
        answer = answer.max(now);
    }
    println!("{}", answer);
}
