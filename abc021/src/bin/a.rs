use proconio::*;

fn main() {
    input! {
        mut n: u32,
    }
    let mut v: Vec<u32> = Vec::new();
    let mut a: u32 = 1;
    while n > 0 {
        if n % 2 == 1 {
            v.push(a);
        }
        n /= 2;
        a *= 2;
    }
    println!("{}", v.len());
    for x in v {
        println!("{}", x);
    }
}
