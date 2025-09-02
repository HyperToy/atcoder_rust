use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [(i64, i64); n],
    }
    let (rs, cs): (Vec<_>, Vec<_>) = xs.into_iter().map(|(x, y)| (x, y)).unzip();
    let r_max = rs.iter().max().unwrap();
    let c_max = cs.iter().max().unwrap();
    let r_min = rs.iter().min().unwrap();
    let c_min = cs.iter().min().unwrap();
    let answer = i64::max((r_max - r_min + 1) / 2, (c_max - c_min + 1) / 2);
    println!("{}", answer);
}
