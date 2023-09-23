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
            C: [[usize; 3]; 3]
        }

        let mut perm = vec![];
        for i in 0..3 {
            for j in 0..3 {
                perm.push((i, j));
            }
        }
        let mut all = 0;
        let mut cnt = 0;
        for p in perm.iter().permutations(perm.len()) {
            all += 1;
            'outer: for i in 0..9 {
                for j in i + 1..9 {
                    for k in j + 1..9 {
                        let (x1, y1) = *p[i];
                        let (x2, y2) = *p[j];
                        let (x3, y3) = *p[k];
                        let a = C[x1][y1];
                        let b = C[x2][y2];
                        let c = C[x3][y3];
                        if a != b || b == c {
                            continue;
                        }
                        if (x1 == x2 && x2 == x3)
                            || (y1 == y2 && y2 == y3)
                            || (x1 == y1 && x2 == y2 && x3 == y3)
                            || (x1 + y1 == 2 && x2 + y2 == 2 && x3 + y3 == 2)
                        {
                            cnt += 1;
                            break 'outer;
                        }
                    }
                }
            }
        }
        let ans = (all as f64 - cnt as f64) / all as f64;
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
