use itertools::Itertools;

fn main() {
    let mut answer = Vec::new();
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let a: i32 = s.trim().parse().unwrap();
        answer.push(a);
        if a == 0 {
            break;
        }
    }
    println!("{}", answer.iter().rev().join(" "));
}
