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
            mut XY: [(isize, isize); N]
        }

        let mut X = vec![];
        let mut Y = vec![];
        for i in 0..N {
            let (x, y) = XY[i];
            XY[i] = (x + y, x - y);
            X.push(x + y);
            Y.push(x - y);
        }
        let xmax = X.iter().max().unwrap();
        let xmin = X.iter().min().unwrap();
        let ymax = Y.iter().max().unwrap();
        let ymin = Y.iter().min().unwrap();
        for _ in 0..Q {
            input! {
                q: Usize1
            }
            let mut v = vec![];
            let (x, y) = XY[q];
            v.push((x - xmax).abs());
            v.push((x - xmin).abs());
            v.push((y - ymax).abs());
            v.push((y - ymin).abs());
            let ans = v.iter().max().unwrap();
            println!("{}", ans);
        }
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
