use itertools::Itertools;
use proconio::input;
use std::ops::Add;

fn main() {
    input! {
        n: usize, q: usize,
        a: [u32; n],
        queries: [(i32, u32, u32); q],
    }
    let mut seg_tree = SegTree::new(n, 0);
    for i in 0..n {
        seg_tree.update(i, a[i]);
    }
    let mut answer = Vec::new();
    for (t, x, y) in queries {
        match t {
            1 => {
                seg_tree.update(x as usize - 1, y);
            }
            2 => {
                answer.push(
                    seg_tree
                        .query(x as usize - 1, y as usize)
                        .second
                        .map_or(0, |(_, count)| count),
                );
            }
            _ => unreachable!(),
        }
    }
    println!("{}", answer.iter().join("\n"));
}

#[derive(Debug, Clone, Copy)]
struct Node {
    first: Option<(u32, usize)>,
    second: Option<(u32, usize)>,
}
impl Node {
    fn from(val: u32) -> Self {
        Self {
            first: Some((val, 1)),
            second: None,
        }
    }
    fn zero() -> Self {
        Self {
            first: None,
            second: None,
        }
    }
    fn insert(&mut self, val: u32, count: usize) {
        match self.first {
            None => {
                self.first = Some((val, count));
            }
            Some((first_val, _)) if val > first_val => {
                self.second = self.first;
                self.first = Some((val, count));
            }
            Some((first_val, _)) if val == first_val => {
                if let Some(ref mut first) = self.first {
                    first.1 += count;
                }
            }
            Some((_, _)) => match self.second {
                None => {
                    self.second = Some((val, count));
                }
                Some((second_val, _)) if val > second_val => {
                    self.second = Some((val, count));
                }
                Some((second_val, _)) if val == second_val => {
                    if let Some(ref mut second) = self.second {
                        second.1 += count;
                    }
                }
                _ => {}
            },
        }
    }
}
impl Add for Node {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut result = Node::zero();
        if let Some((val, count)) = self.first {
            result.insert(val, count);
        }
        if let Some((val, count)) = self.second {
            result.insert(val, count);
        }
        if let Some((val, count)) = other.first {
            result.insert(val, count);
        }
        if let Some((val, count)) = other.second {
            result.insert(val, count);
        }
        result
    }
}
#[derive(Debug)]
struct SegTree {
    data: Vec<Node>,
    size: usize,
}
impl SegTree {
    fn new(n: usize, init_val: u32) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let data = vec![Node::from(init_val); size * 2];
        Self { data, size }
    }
    fn update(&mut self, pos: usize, val: u32) {
        let mut pos = pos + self.size;
        self.data[pos] = Node::from(val);
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2] + self.data[pos * 2 + 1];
        }
    }
    fn query_sub(&self, l: usize, r: usize, a: usize, b: usize, k: usize) -> Node {
        if r <= a || b <= l {
            return Node {
                first: None,
                second: None,
            };
        }
        if l <= a && b <= r {
            return self.data[k];
        }
        let m = (a + b) / 2;
        let val_l = self.query_sub(l, r, a, m, k * 2);
        let val_r = self.query_sub(l, r, m, b, k * 2 + 1);
        val_l + val_r
    }
    fn query(&self, l: usize, r: usize) -> Node {
        self.query_sub(l, r, 0, self.size, 1)
    }
}
