use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }
    let mul = a.iter().product::<f64>();
    println!("{}", mul / a.iter().map(|x| mul / x).sum::<f64>());
}
