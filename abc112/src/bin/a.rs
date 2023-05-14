use proconio::*;

fn main() {
    input! {
        n: u32,
    }

    match n  {
        1 => println!("Hello World"),
        2 => {
            input! {
                a: u32, b: u32,
            }
            println!("{}", a + b);
        }
        _ => unreachable!()
    }
}
