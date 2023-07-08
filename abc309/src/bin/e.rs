#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::marker::Isize1;
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
            M: usize,
            P: [Usize1; N - 1],
            XY: [(Usize1, isize); M]
        }

        let mut G = vec![vec![]; N];
        for (i, &p) in P.iter().enumerate() {
            G[p].push(i + 1);
        }

        let mut comp = vec![0; N];
        for &(x, y) in &XY {
            comp[x] = max!(comp[x], y);
        }
        // println!("{:?}", comp);
        let mut d = vec![-1; N];
        let mut Q = BinaryHeap::new();
        for (i, &c) in comp.iter().enumerate() {
            if c == 0 {
                continue;
            }
            Q.push((c, i));
            d[i] = c;
        }

        while !Q.is_empty() {
            let (_, pos) = Q.pop().unwrap();
            for &next in &G[pos] {
                if d[next] < d[pos] - 1 {
                    d[next] = d[pos] - 1;
                    Q.push((d[next], next));
                }
            }
        }
        // println!("{:?}", d);
        let ans = d.iter().filter(|&&x| x != -1).count();
        println!("{}", ans);
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
