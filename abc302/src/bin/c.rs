use itertools::Itertools;
use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    }

    let check = |p: &Vec<char>, q: &Vec<char>| {
        let mut diff_count = 0;
        for j in 0..m {
            if p[j] != q[j] {
                diff_count += 1;
            }
        }
        diff_count == 1
    };

    // 順列全探索
    let mut answer = "No";
    for t in s.iter().permutations(n) {
        let mut ok = true;
        for i in 0..n - 1 {
            if !check(t[i], t[i + 1]) {
                ok = false;
            }
        }
        if ok {
            answer = "Yes";
        }
    }

    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn how_permutations_works() {
        let vec = vec![2, 3, 1];
        for v in vec.iter().permutations(3) {
            eprintln!("{:?}", v);
        }
        /* result
        [2, 3, 1]
        [2, 1, 3]
        [3, 2, 1]
        [3, 1, 2]
        [1, 2, 3]
        [1, 3, 2]
        */
    }
}
