use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let max = *a.iter().max().unwrap();
    let mut count = vec![(0, 0); max * 2 + n];
    a.iter()
        .enumerate()
        .map(|(i, x)| (max + i + *x, max + i - *x))
        .for_each(|(x, y)| {
            count[x].0 += 1;
            count[y].1 += 1
        });
    println!("{}", count.iter().map(|(x, y)| x * y).sum::<usize>());
}
