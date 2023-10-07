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
            A: [usize; N]
        }

        let mut ascending = vec![0; N];
        let mut descending = vec![0; N];
        ascending[0] = 1;
        descending[N - 1] = 1;

        let mut cnt = 1;
        let mut cnt_max = 1;
        for i in 1..N {
            if A[i - 1] < A[i] {
                cnt += 1;
            } else {
                cnt = 1;
            }
            cnt_max = max!(cnt_max, cnt);
            ascending[i] = cnt_max;
        }

        cnt = 1;
        cnt_max = 1;
        for i in (0..N - 1).rev() {
            if A[i] > A[i + 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }
            cnt_max = max!(cnt_max, cnt);
            descending[i] = cnt_max;
        }
        let mut ans = 0_usize;
        for i in 0..N {
            ans = max!(ans, ascending[i] + descending[i] - 1);
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
