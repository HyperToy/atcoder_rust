use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        a: [Usize1; m],
    }
    let mut count = vec![0; n];
    let mut winner = None;
    for x in a {
        count[x] += 1;
        if let Some(w) = winner {
            if count[x] > count[w] || count[x] == count[w] && x < w {
                winner = Some(x);
            }
        } else {
            winner = Some(x);
        }
        println!("{}", winner.unwrap() + 1);
    }
}
