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
            K: usize,
            A: [usize; N]
        }

        fn f(m: usize, A: &Vec<usize>, K: usize) -> bool {
            let mut cnt = 0_usize;
            for a in A {
                if *a >= m {
                    cnt += a - m + 1;
                }
            }
            K >= cnt
        }

        let mut ng = 0;
        let mut ok = 2e9 as usize;
        while ok - ng > 1 {
            let m = (ok + ng) / 2;
            if f(m, &A, K) {
                ok = m;
            } else {
                ng = m;
            }
        }

        let mut ans = 0_usize;
        let mut cnt = 0_usize;
        for &a in &A {
            if a >= ok {
                let c = a - ok + 1;
                cnt += c;
                ans += c * (ok + a) / 2;
            }
        }

        if cnt < K {
            ans += (ok - 1) * (K - cnt);
        }
        println!("{}", ans);
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
