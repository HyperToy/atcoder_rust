use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
    }
    // judge[i][j] := a[i] < a[j] ?
    let mut judge = vec![vec![None; n * 2]; n * 2];
    let mut max = 0;
    for i in 1..n {
        println!("? {} {}", max, i);
        input! {
            from &mut source,
            t: i32,
        }
        let res = match t {
            0 => false,
            1 => true,
            _ => unreachable!(),
        };
        judge[max][i] = Some(res);
        judge[i][max] = Some(!res);
        if res {
            max = i;
        }
    }
    todo!();
}
