use proconio::input;

fn main() {
    input! {
        n: usize,
        as_: [(i32, char); n],
    }
    let mut answer = 0;
    let mut pl = None;
    let mut pr = None;
    for (a, s) in as_ {
        let pre = match s {
            'L' => &mut pl,
            'R' => &mut pr,
            _ => unreachable!(),
        };
        answer += match pre {
            None => 0,
            Some(x) => (&*x - a).abs(),
        };
        *pre = Some(a);
    }
    println!("{}", answer);
}
