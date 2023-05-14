use proconio::*;

fn main() {
    input! {
        x: u32,
        y: u32,
    }
    let mut ans: u32 = 0;
    ans += match x {
        1 => 300000,
        2 => 200000,
        3 => 100000,
        _ => 0,
    };
    ans += match y {
        1 => 300000,
        2 => 200000,
        3 => 100000,
        _ => 0,
    };
    ans += if x == 1 && y == 1 {
        400000
    } else {
        0
    };
    println!("{}", ans);
}
