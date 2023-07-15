use proconio::{
    marker::{Bytes, Usize1},
    *,
};

fn main() {
    input! {
        n: usize, q: usize,
        s: Bytes,
        qs: [(Usize1, Usize1); q],
    }
    let modulo = 1_000_000_000 + 7;
    let mut pow27 = vec![1; n];
    for i in 1..n {
        pow27[i] = pow27[i - 1] * 27 % modulo;
    }

    let b: Vec<_> = s.iter().map(|x| x - b'a' + 1).collect();

    let mut hash = vec![0; n];
    for i in 0..n {
        hash[i] = (b[i] as i64 + if i > 0 { hash[i - 1] * 27 } else { 0 }) % modulo;
    }
    let convert =
        |l, r| (hash[r] - if l > 0 { hash[l - 1] } else { 0 } * pow27[r - l + 1] + modulo) % modulo;

    let mut r_hash = vec![0; n];
    for i in (0..n).rev() {
        r_hash[i] = (b[i] as i64 + if i < n - 1 { r_hash[i + 1] * 27 } else { 0 }) % modulo;
    }
    let reverse_convert = |l, r| {
        (r_hash[l] - if r < n - 1 { r_hash[r + 1] } else { 0 } * pow27[r - l + 1] + modulo) % modulo
    };

    for (l, r) in qs {
        println!(
            "{}",
            if convert(l, r) == reverse_convert(l, r) {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
