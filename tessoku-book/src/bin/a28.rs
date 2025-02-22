use proconio::*;

fn main() {
    input! {
        n: usize,
        qs: [(char, u32); n],
    }
    let mut x = 0;
    let modulo = 10000;
    for (t, a) in qs {
        x = match t {
            '+' => x + a,
            '-' => x + modulo - a,
            '*' => x * a,
            _ => unreachable!(),
        } % modulo;
        println!("{}", x);
    }
}
