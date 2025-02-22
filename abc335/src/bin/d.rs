use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut b = vec![vec!["".to_string(); n]; n];
    b[(n - 1) / 2][(n - 1) / 2] = "T".to_string();
    let mut p = (0, 0);
    let mut d = (0, 1);
    let mut c = 1;
    for _ in 0..n * n - 1 {
        b[p.0 as usize][p.1 as usize] = c.to_string();
        let mut np = (p.0 + d.0, p.1 + d.1);
        if !inside(np, n) || b[np.0 as usize][np.1 as usize] != "".to_string() {
            d = match d {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => unreachable!(),
            };
            np = (p.0 + d.0, p.1 + d.1);
        }
        p = np;
        c += 1;
    }
    println!("{}", b.iter().map(|r| r.iter().join(" ")).join("\n"));
}
fn inside(p: (isize, isize), n: usize) -> bool {
    0 <= p.0 && p.0 < n as isize && 0 <= p.1 && p.1 < n as isize
}
