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
            A: i128,
            M: i128,
            L: i128,
            R: i128
        }

        let mut ng = -5e18 as i128;
        let mut ok = 5e18 as i128;
        while (ok - ng) > 1 {
            let m = (ok + ng) / 2;
            if A + m * M >= L {
                ok = m;
            } else {
                ng = m;
            }
        }

        let l = A + ok * M;

        let mut ok = -5e18 as i128;
        let mut ng = 5e18 as i128;
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            if A + m * M <= R {
                ok = m;
            } else {
                ng = m;
            }
        }

        let r = A + ok * M;

        println!("{}", (r - l) / M + 1);
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
