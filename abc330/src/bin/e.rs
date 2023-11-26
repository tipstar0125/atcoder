#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use superslice::Ext;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            Q: usize,
            mut A: [usize; N],
        }

        let mut seg = SegmentTree::<MinMonoid>::new(N + 1);
        for i in 0..=N {
            seg.update(i, (0, i));
        }
        for &a in &A {
            if a <= N {
                let (cnt, pos) = seg.get(a);
                assert!(a == pos);
                seg.update(a, (cnt + 1, pos));
            }
        }

        for _ in 0..Q {
            input! {
                i: Usize1,
                x: usize
            }
            if A[i] <= N {
                let (cnt, y) = seg.get(A[i]);
                assert!(A[i] == y);
                assert!(cnt > 0);
                seg.update(A[i], (cnt - 1, y));
            }
            if x <= N {
                let (cnt, y) = seg.get(x);
                assert!(x == y);
                seg.update(x, (cnt + 1, y));
            }
            A[i] = x;
            let (_, ans) = seg.query(0, N + 1);
            println!("{}", ans);
        }
    }
}

#[derive(Debug, Clone)]
struct SegmentTree<M>
where
    M: Monoid,
{
    offset: usize,
    data: Vec<M::S>,
}

impl<M> SegmentTree<M>
where
    M: Monoid,
    M::S: Clone,
{
    fn new(n: usize) -> Self {
        let offset = n.next_power_of_two();
        let data = vec![M::id(); offset * 2 - 1];
        SegmentTree { offset, data }
    }
    // pos: 0-indexed!!
    fn update(&mut self, pos: usize, x: M::S) {
        let mut pos = pos + self.offset - 1;
        self.data[pos] = x;
        while pos > 0 {
            pos = (pos - 1) / 2;
            self.data[pos] = M::op(&self.data[pos * 2 + 1], &self.data[pos * 2 + 2]);
        }
    }
    // pos: 0-indexed!!
    fn get(&self, pos: usize) -> M::S {
        self.data[self.offset + pos - 1].clone()
    }
    // 0-indexed!!, [l, r)
    fn query(&self, mut l: usize, mut r: usize) -> M::S {
        l += self.offset - 1;
        r += self.offset - 1;
        let mut xl = M::id();
        let mut xr = M::id();

        while l < r {
            if l % 2 == 0 {
                xl = M::op(&xl, &self.data[l]);
            }
            if r % 2 == 0 {
                xr = M::op(&xr, &self.data[r - 1]);
            }
            l /= 2;
            r = (r - 1) / 2;
        }
        M::op(&xl, &xr)
    }
}

trait Monoid {
    type S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    fn id() -> Self::S;
}

struct MinMonoid;

impl Monoid for MinMonoid {
    type S = (usize, usize);
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::min(*a, *b)
    }
    fn id() -> Self::S {
        (std::usize::MAX, std::usize::MAX)
    }
}

struct MaxMonoid;

impl Monoid for MaxMonoid {
    type S = usize;
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::max(*a, *b)
    }
    fn id() -> Self::S {
        0
    }
}

struct AddMonoid;

impl Monoid for AddMonoid {
    type S = usize;
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
    fn id() -> Self::S {
        0
    }
}

struct MulMonoid;

impl Monoid for MulMonoid {
    type S = usize;
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        *a * *b
    }
    fn id() -> Self::S {
        1
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
