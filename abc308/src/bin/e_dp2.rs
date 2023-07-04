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
            A: [usize; N],
            S: Chars
        }

        let mex = |m: usize, e: usize, x: usize| -> usize {
            let mut set = BTreeSet::new();
            set.insert(m);
            set.insert(e);
            set.insert(x);
            let mut res = 0;
            for i in 0..=3 {
                if set.contains(&i) {
                    continue;
                }
                res = i;
                break;
            }
            res
        };

        let mut ans = 0_usize;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let m = mex(i, j, k);
                    let mut dp = vec![0; 3];
                    for l in 1..=N {
                        let a = A[l - 1];
                        let s = S[l - 1];
                        if s == 'M' && a == i {
                            dp[0] += 1;
                        }
                        if s == 'E' && a == j {
                            dp[1] += dp[0];
                        }
                        if s == 'X' && a == k {
                            dp[2] += dp[1];
                        }
                    }
                    ans += m * dp[2];
                }
            }
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
