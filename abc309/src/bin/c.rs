use proconio::*;

fn main() {
    input! {
        n: usize, k: usize,
        mut medicines: [(usize, usize); n],
    }
    let mut x = medicines.iter().map(|(_, x)| x).sum::<usize>();
    medicines.sort_by(|a, b| a.0.cmp(&b.0));
    let mut day = 1;
    let mut idx = 0;
    while x > k {
        day = medicines[idx].0 + 1;
        x -= medicines[idx].1;
        idx += 1;
    }
    println!("{}", day);
}
