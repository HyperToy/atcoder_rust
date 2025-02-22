use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: u32,
        rs: [u32; n],
    }
    println!(
        "{}",
        rec(&rs, k, &mut vec![])
            .iter()
            .map(|v| v.iter().join(" "))
            .join("\n")
    );
}
fn rec(rs: &Vec<u32>, k: u32, now: &mut Vec<u32>) -> Vec<Vec<u32>> {
    if now.len() == rs.len() {
        if now.iter().sum::<u32>() % k == 0 {
            return vec![now.clone()];
        } else {
            return vec![];
        }
    }
    let mut res = vec![];
    for i in 1..=rs[now.len()] {
        now.push(i);
        res.append(&mut rec(rs, k, now));
        now.pop();
    }
    res
}
