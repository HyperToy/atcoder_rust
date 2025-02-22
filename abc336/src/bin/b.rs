use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    println!("{}", n.trailing_zeros());
}
