use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, q: usize,
    }
    let mut rmq = SegTree::new(n, 0);
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    pos: Usize1, x: i32,
                }
                rmq.update(pos, x);
            }
            2 => {
                input! {
                    l: Usize1, r: Usize1,
                }
                println!("{}", rmq.query(l, r));
            }
            _ => unreachable!(),
        }
    }
}

struct SegTree {
    data: Vec<i32>,
    size: usize,
}
impl SegTree {
    fn new(n: usize, init_val: i32) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let data = vec![init_val; size * 2];
        Self { data, size }
    }
    fn update(&mut self, pos: usize, val: i32) {
        let mut pos = pos + self.size;
        self.data[pos] = val;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2].max(self.data[pos * 2 + 1]);
        }
    }
    fn query_sub(&self, l: usize, r: usize, a: usize, b: usize, k: usize) -> i32 {
        if r <= a || b <= l {
            return -1_000_000_000;
        }
        if l <= a && b <= r {
            return self.data[k];
        }
        let m = (a + b) / 2;
        let val_l = self.query_sub(l, r, a, m, k * 2);
        let val_r = self.query_sub(l, r, m, b, k * 2 + 1);
        val_l.max(val_r)
    }
    fn query(&self, l: usize, r: usize) -> i32 {
        self.query_sub(l, r, 0, self.size, 1)
    }
}
