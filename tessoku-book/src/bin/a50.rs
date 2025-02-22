#![allow(unused_variables)]

use proconio::*;
use rand::Rng;

fn main() {
    let n = 100;
    let q = 1000;
    input! {
        a: [[i32; n]; n],
    }
    let mut answer = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..q {
        let y = rng.gen_range(0..100);
        let x = rng.gen_range(0..100);
        let h = rng.gen_range(0..50);
        answer.push((Point { y, x }, h));
    }

    print_output(&answer);
}

struct Point {
    y: i32,
    x: i32,
}

fn print_output(answer: &Vec<(Point, i32)>) {
    println!("{}", answer.len());
    for (p, h) in answer {
        println!("{} {} {}", p.x, p.y, h);
    }
}
