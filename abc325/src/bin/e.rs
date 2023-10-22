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
            A: usize,
            B: usize,
            C: usize,
            D: [[usize; N]; N]
        }

        let INF = 1_usize << 60;
        let mut dist1 = vec![INF; N];
        let mut Q = BinaryHeap::new();
        dist1[0] = 0;
        Q.push((Reverse(dist1[0]), 0));
        while let Some((Reverse(d), pos)) = Q.pop() {
            if dist1[pos] != d {
                continue;
            }
            for next in 0..N {
                if pos == next {
                    continue;
                }
                if dist1[pos] + D[pos][next] * A < dist1[next] {
                    dist1[next] = dist1[pos] + D[pos][next] * A;
                    Q.push((Reverse(dist1[next]), next));
                }
            }
        }

        let mut dist2 = vec![INF; N];
        let mut Q = BinaryHeap::new();
        dist2[N - 1] = 0;
        Q.push((Reverse(dist2[N - 1]), N - 1));
        while let Some((Reverse(d), pos)) = Q.pop() {
            if dist2[pos] != d {
                continue;
            }
            for next in 0..N {
                if pos == next {
                    continue;
                }
                if dist2[pos] + D[pos][next] * B + C < dist2[next] {
                    dist2[next] = dist2[pos] + D[pos][next] * B + C;
                    Q.push((Reverse(dist2[next]), next));
                }
            }
        }

        let mut ans = INF;
        for i in 0..N {
            ans = min!(ans, dist1[i] + dist2[i]);
        }
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
