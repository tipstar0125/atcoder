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
            mut HWD: [[usize; 3]; N]
        }

        let mut v = vec![];
        for i in 0..N {
            for j in 0..3 {
                v.push(HWD[i][j]);
            }
        }
        let mp = coordinate_compression(v);
        let mut XYZ = vec![];
        for i in 0..N {
            HWD[i].sort();
            XYZ.push((mp[&HWD[i][0]], mp[&HWD[i][1]], mp[&HWD[i][2]]));
        }
        XYZ.sort_by(|a, b| (b.0, a.1).cmp(&(a.0, b.1)));

        let L = mp.len();
        let mut seg = SegmentTree::new(L);

        for i in 0..N {
            let (_, y1, z1) = XYZ[i];
            if y1 + 1 < L {
                let y = y1 + 1;
                let z = seg.query(y + 1, L + 1, 1, seg.offset + 1, 1) as usize;
                if z > z1 {
                    println!("Yes");
                    return;
                }
            }
            let now = seg.get(y1) as usize;
            if now < z1 {
                seg.set(y1, z1 as isize);
            }
        }
        println!("No");
    }
}

fn coordinate_compression<T: std::cmp::Ord + Copy>(v: Vec<T>) -> BTreeMap<T, usize> {
    let mut vv = v;
    vv.sort();
    vv.dedup();
    let ret = vv.iter().enumerate().map(|(i, &s)| (s, i)).collect();
    ret
}

#[derive(Debug, Clone)]
struct SegmentTree {
    offset: usize,
    data: Vec<isize>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut offset = 1;
        while offset < n {
            offset *= 2;
        }
        let data = vec![0; offset * 2];
        SegmentTree { offset, data }
    }
    // pos: 0-indexed!!
    fn set(&mut self, pos: usize, x: isize) {
        let mut pos = pos + self.offset;
        self.data[pos] = x;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2].max(self.data[pos * 2 + 1]);
        }
    }
    // pos: 0-indexed!!
    fn get(&self, pos: usize) -> isize {
        self.data[self.offset + pos]
    }
    // u: current cell number
    // 1-indexed!!, [l, r), [a, b)
    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> isize {
        if r <= a || b <= l {
            return -1_isize << 60;
        }
        if l <= a && b <= r {
            return self.data[u];
        }
        let m = (a + b) / 2;
        self.query(l, r, a, m, u * 2)
            .max(self.query(l, r, m, b, u * 2 + 1))
    }
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::max($x, max!($( $y ),+))
    }
}
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::min($x, min!($( $y ),+))
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
