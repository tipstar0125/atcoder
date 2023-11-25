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
            S: [Chars; N]
        }

        let mut row_S = vec![0; N];
        for i in 0..N {
            let mut s = 0;
            for j in 0..N {
                if S[i][j] == 'o' {
                    s += 1;
                }
            }
            row_S[i] = s;
        }
        let mut col_S = vec![0; N];
        for j in 0..N {
            let mut s = 0;
            for i in 0..N {
                if S[i][j] == 'o' {
                    s += 1;
                }
            }
            col_S[j] = s;
        }

        let mut ans = 0_usize;
        for i in 0..N {
            for j in 0..N {
                if S[i][j] == 'x' {
                    continue;
                }
                if row_S[i] < 1 {
                    continue;
                }
                if col_S[j] < 1 {
                    continue;
                }
                ans += (row_S[i] - 1) * (col_S[j] - 1);
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
