use itertools::Itertools;
use proconio::input_interactive;
use std::{cmp::Ordering, process::exit};

fn main() {
    input_interactive! {
        n: usize,
    }
    let mut indices = (1..=n).collect_vec();
    indices.sort_by(|&i, &j| {
        println!("? {} {}", i, j);
        input_interactive! {
            result: i32,
        }
        if result == -1 {
            exit(-1);
        } else if result == 1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    while indices.len() > 1 {
        let i = indices[0];
        let j = indices.last().unwrap();
        println!("+ {} {}", i, j);
        input_interactive! {
            result: i32,
        }
        if result == -1 {
            exit(-1);
        }
        indices.pop();
        indices.remove(0);
        let p = result as usize;
        let pos = indices.binary_search_by(|&x| {
            println!("? {} {}", x, p);
            input_interactive! {
                result: i32,
            }
            if result == -1 {
                exit(-1);
            } else if result == 1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let pos = match pos {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        indices.insert(pos, p);
    }
    println!("!");
}
