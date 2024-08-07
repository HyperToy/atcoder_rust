use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut answer = 0;
    for d in 0..30 {
        let b = a.iter().map(|&x| (x >> d) & 1).collect_vec();
        let mut c = vec![0];
        for &x in &b {
            c.push(c.last().unwrap() ^ x);
        }
        let now = c.iter().filter(|&&x| x == 0).count() * c.iter().filter(|&&x| x == 1).count()
            - b.iter().sum::<usize>();
        answer += now << d;
    }
    println!("{}", answer);
}
