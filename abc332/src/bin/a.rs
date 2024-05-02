use proconio::input;

fn main() {
    input! {
        n: usize, s: i32, k: i32,
        pq: [(i32, i32); n],
    }
    let sum = pq.iter().map(|(p, q)| p * q).sum::<i32>();
    println!("{}", sum + if sum >= s { 0 } else { k });
}
