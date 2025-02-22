use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sc: [(String, usize); n],
    }
    sc.sort();
    let sum = sc.iter().map(|(_, c)| c).sum::<usize>();
    println!("{}", sc[sum % n].0);
}
