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
            _N: usize,
            mut S: Chars,
            Q: usize,
            TXC: [(usize, usize, char); Q]
        }

        let mut tt = -1;
        let mut size = 0;
        for i in (0..Q).rev() {
            let (t, _, _) = TXC[i];
            if t != 1 {
                tt = i as isize;
                size = t;
                break;
            }
        }

        for (i, &(t, x, c)) in TXC.iter().enumerate() {
            if t == 1 {
                S[x - 1] = c;
            }
            if i as isize == tt {
                if size == 2 {
                    let mut T = vec![];
                    for s in S {
                        T.push(s.to_lowercase().to_string().chars().collect::<Vec<char>>()[0]);
                    }
                    S = T;
                } else {
                    let mut T = vec![];
                    for s in S {
                        T.push(s.to_uppercase().to_string().chars().collect::<Vec<char>>()[0]);
                    }
                    S = T;
                }
            }
        }
        println!("{}", S.iter().join(""));
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
