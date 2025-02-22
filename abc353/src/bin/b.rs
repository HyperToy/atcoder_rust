use proconio::input;

fn main() {
    input! {
        n: usize, k: i32,
        a: [i32; n],
    }
    let mut rem = k;
    let mut answer = 0;
    for x in a {
        if x <= rem {
            rem -= x;
        } else {
            rem = k - x;
            answer += 1;
        }
    }
    println!("{}", answer + 1);
}
