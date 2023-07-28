use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
    }
    let mut goods = Vec::new();
    for _ in 0..n {
        input! {
            price: usize, c: usize, f: [Usize1; c],
        }
        let mut functions = vec![false; m];
        for i in 0..c {
            functions[f[i]] = true;
        }
        goods.push(Good { price, functions });
    }
    let mut exists = false;
    for i in 0..n {
        for j in 0..n {
            let mut ok = true;
            ok &= goods[i].price >= goods[j].price;
            let mut i_count = 0;
            let mut j_count = 0;
            for k in 0..m {
                ok &= !goods[i].functions[k] || goods[j].functions[k];
                i_count += if goods[i].functions[k] { 1 } else { 0 };
                j_count += if goods[j].functions[k] { 1 } else { 0 };
            }
            ok &= goods[i].price > goods[j].price || i_count < j_count;
            exists |= ok;
        }
    }
    println!("{}", if exists { "Yes" } else { "No" });
}

struct Good {
    price: usize,
    functions: Vec<bool>,
}
