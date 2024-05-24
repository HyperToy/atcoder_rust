use crate::procon_lib::MultiSet;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        a: [i64; n],
    }
    let mut solver = Solver::new(k);
    let mut answer = Vec::new();
    for i in 0..n {
        solver.add(a[i]);
        if solver.size() == m {
            answer.push(solver.query());
            solver.remove(a[i - (m - 1)]);
        }
    }
    println!("{}", answer.iter().join(" "));
}

struct Solver {
    sum: i64,
    k: usize,
    set1: MultiSet<i64>,
    set2: MultiSet<i64>,
}
impl Solver {
    fn new(k: usize) -> Self {
        Self {
            sum: 0,
            k,
            set1: MultiSet::new(),
            set2: MultiSet::new(),
        }
    }
    fn add(&mut self, x: i64) {
        self.set2.insert(x);
    }
    fn remove(&mut self, x: i64) {
        if self.set1.contains(&x) {
            self.set1.remove(&x);
            self.sum -= x;
        } else if self.set2.contains(&x) {
            self.set2.remove(&x);
        } else {
            unreachable!();
        }
    }
    fn balance(&mut self) {
        while self.set1.size() < self.k {
            let y = *self.set2.first().unwrap();
            self.set2.remove(&y);
            self.sum += y;
            self.set1.insert(y);
        }
        while self.set1.size() > self.k {
            let x = *self.set1.last().unwrap();
            self.set1.remove(&x);
            self.sum -= x;
            self.set2.insert(x);
        }
        if !self.set1.is_empty() && !self.set2.is_empty() {
            while self.set1.last().unwrap() > self.set2.first().unwrap() {
                let x = *self.set1.last().unwrap();
                let y = *self.set2.first().unwrap();
                self.set1.remove(&x);
                self.set2.remove(&y);
                self.set1.insert(y);
                self.set2.insert(x);
                self.sum -= x;
                self.sum += y;
            }
        }
    }
    fn query(&mut self) -> i64 {
        self.balance();
        self.sum
    }
    fn size(&self) -> usize {
        self.set1.size() + self.set2.size()
    }
}

// TODO: snippet
#[allow(dead_code)]
mod procon_lib {
    use std::{
        borrow::Borrow,
        collections::{
            btree_map::{self},
            BTreeMap,
        },
        ops::{Bound, RangeBounds},
    };

    #[derive(Debug, PartialEq, Clone)]
    pub struct MultiSet<T> {
        size: usize,
        btree_map: BTreeMap<T, usize>,
    }

    impl<T: Ord> Default for MultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Ord> From<Vec<T>> for MultiSet<T> {
        fn from(value: Vec<T>) -> Self {
            let size = value.len();

            let mut btree_map = BTreeMap::default();
            for key in value {
                *btree_map.entry(key).or_insert(0) += 1;
            }

            Self { size, btree_map }
        }
    }

    impl<T: Ord> MultiSet<T> {
        pub fn clear(&mut self) {
            self.size = 0;
            self.btree_map.clear();
        }

        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.btree_map.contains_key(value)
        }

        pub fn insert(&mut self, value: T) {
            self.size += 1;
            *self.btree_map.entry(value).or_insert(0) += 1;
        }

        pub fn first(&self) -> Option<&T> {
            if let Some((key, _)) = self.btree_map.iter().next() {
                Some(key)
            } else {
                None
            }
        }

        pub fn last(&self) -> Option<&T> {
            if let Some((key, _)) = self.btree_map.iter().next_back() {
                Some(key)
            } else {
                None
            }
        }

        pub fn is_empty(&self) -> bool {
            self.btree_map.is_empty()
        }

        // キーの要素数
        pub fn len(&self) -> usize {
            self.btree_map.len()
        }

        // 重複を含めた要素数
        pub fn size(&self) -> usize {
            self.size
        }

        pub fn marge(&mut self, other: &mut MultiSet<T>)
        where
            T: Clone,
        {
            self.size += other.size;

            for (key, val) in other.btree_map.iter() {
                if let Some(prev) = self.btree_map.get_mut(key) {
                    *prev += *val;
                } else {
                    self.btree_map.insert(key.clone(), *val);
                }
            }
        }

        pub fn new() -> Self {
            Self {
                size: 0,
                btree_map: BTreeMap::new(),
            }
        }

        pub fn pop_first(&mut self) -> Option<T>
        where
            T: Clone,
        {
            if self.is_empty() {
                None
            } else {
                let first = self.first().unwrap().clone();
                self.remove(&first);
                Some(first)
            }
        }

        pub fn pop_last(&mut self) -> Option<T>
        where
            T: Clone,
        {
            if self.is_empty() {
                None
            } else {
                let last = self.last().unwrap().clone();
                self.remove(&last);
                Some(last)
            }
        }

        pub fn remove(&mut self, value: &T) -> bool
        where
            T: Clone,
        {
            self.btree_map.entry(value.clone()).and_modify(|e| *e -= 1);
            if let Some(&cnt) = self.btree_map.get(&value) {
                if cnt == 0 {
                    self.btree_map.remove(&value);
                }

                self.size -= 1;
                true
            } else {
                false
            }
        }

        pub fn lower_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            match bound {
                Bound::Unbounded => unreachable!(),
                _ => {
                    if let Some((key, _)) = self.btree_map.range((bound, Bound::Unbounded)).next() {
                        Some(key)
                    } else {
                        None
                    }
                }
            }
        }

        pub fn upper_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            match bound {
                Bound::Unbounded => unreachable!(),
                _ => {
                    if let Some((key, _)) =
                        self.btree_map.range((Bound::Unbounded, bound)).next_back()
                    {
                        Some(key)
                    } else {
                        None
                    }
                }
            }
        }

        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                range: self.range(..),
            }
        }

        pub fn range<U: ?Sized, R>(&self, range: R) -> Range<'_, T>
        where
            U: Ord,
            T: Borrow<U> + Ord,
            R: RangeBounds<U>,
        {
            Range {
                last: None,
                counter: 0,
                range: self.btree_map.range(range),
            }
        }

        pub fn count<Q>(&self, value: &Q) -> usize
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            if let Some(&cnt) = self.btree_map.get(value) {
                cnt
            } else {
                0
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct Range<'a, T>
    where
        T: 'a,
    {
        last: Option<&'a T>,
        counter: usize,
        range: btree_map::Range<'a, T, usize>,
    }

    impl<'a, T> Iterator for Range<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.counter == 0 {
                if let Some((elem, &cnt)) = self.range.next() {
                    self.last = Some(elem);
                    self.counter = cnt - 1;
                    Some(elem)
                } else {
                    None
                }
            } else {
                self.counter -= 1;
                self.last
            }
        }
    }

    impl<'a, T> DoubleEndedIterator for Range<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.counter == 0 {
                if let Some((elem, &cnt)) = self.range.next_back() {
                    self.last = Some(elem);
                    self.counter = cnt;
                    Some(elem)
                } else {
                    None
                }
            } else {
                self.counter -= 1;
                self.last
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Iter<'a, T>
    where
        T: 'a,
    {
        range: Range<'a, T>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.range.next()
        }
    }

    impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.range.next_back()
        }
    }
}
