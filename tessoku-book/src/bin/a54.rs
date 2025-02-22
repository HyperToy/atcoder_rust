use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        q: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: String, y: i32,
                }
                map.insert(x, y);
            }
            2 => {
                input! {
                    x: String,
                }
                println!("{}", map.get(&x).unwrap());
            }
            _ => unreachable!(),
        }
    }
}
