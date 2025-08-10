use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let answer = a
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .sorted_by_key(|(a, _)| *a)
        // 各 a が現れる位置をグループ化
        .group_by(|(a, _)| *a)
        .into_iter()
        .map(|(_, positions)| {
            // index をずらして、先頭と末尾に番兵を追加
            std::iter::once(0)
                .chain(positions.map(|(_, i)| i + 1))
                .chain(std::iter::once(n + 1))
        })
        .map(|positions| {
            // 各 a が含まれない連続部分列の個数
            positions
                .tuple_windows()
                .map(|(a, b)| (b - 1) - a)
                .map(|x| x * (x + 1) / 2)
                .sum::<usize>()
        })
        .map(|x| n * (n + 1) / 2 - x) // 各 a が含まれる連続部分列の個数
        .sum::<usize>();
    println!("{}", answer);
}
