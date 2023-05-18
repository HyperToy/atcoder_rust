use proconio::*;

fn main() {
    input! {
        n: usize,
        mountains: [(String, i32); n],
    }
    let mut first = (String::new(), 0);
    let mut second = (String::new(), 0);
    for mountain in mountains {
        if mountain.1 > first.1 {
            second = first;
            first = mountain;
        } else if mountain.1 > second.1 {
            second = mountain;
        }
    }
    println!("{}", second.0);
}
