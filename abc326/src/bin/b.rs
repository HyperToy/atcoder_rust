use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    for i in n..=919 {
        if is_326like_number(i) {
            println!("{}", i);
            break;
        }
    }
}
fn is_326like_number(n: i32) -> bool {
    let a = n / 100;
    let b = n / 10 % 10;
    let c = n % 10;
    a * b == c
}
