use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    let mut answer = 0;
    while a.iter().filter(|&&x| x > 0).count() > 1 {
        answer += 1;
        a.sort();
        a.reverse();
        a[0] -= 1;
        a[1] -= 1;
    }
    println!("{}", answer);
}
