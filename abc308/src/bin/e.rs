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

        let mut S_m = vec![vec![0; 3]; N];
        for i in 0..N {
            if i != 0 {
                for j in 0..3 {
                    S_m[i][j] = S_m[i - 1][j];
                }
            }
            if S[i] == 'M' {
                S_m[i][A[i]] += 1;
            }
        }
        let mut S_x = vec![vec![0; 3]; N];
        for i in (0..N).rev() {
            if i != N - 1 {
                for j in 0..3 {
                    S_x[i][j] = S_x[i + 1][j];
                }
            }
            if S[i] == 'X' {
                S_x[i][A[i]] += 1;
            }
        }

        let mut ans = 0_usize;
        for i in 0..N {
            if S[i] != 'E' {
                continue;
            }
            let y = A[i];
            for x in 0..3 {
                for z in 0..3 {
                    let m = mex(x, y, z);
                    ans += m * S_m[i][x] * S_x[i][z];
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
