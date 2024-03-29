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
            T: Chars,
            S: [Chars; N]
        }

        let mut A = vec![];
        let mut B = vec![];
        let mut T_rev = T.clone();
        T_rev.reverse();

        for s in S.iter() {
            let mut i = 0;
            for si in s {
                if *si == T[i] {
                    i += 1;
                }
                if i == T.len() {
                    break;
                }
            }
            A.push(i);
            let mut i = 0;
            for si in s.iter().rev() {
                if *si == T_rev[i] {
                    i += 1;
                }
                if i == T.len() {
                    break;
                }
            }
            B.push(i);
        }
        B.sort();
        let mut ans = 0_usize;
        for a in A {
            let num = N - B.lower_bound(&(T.len() - a));
            ans += num;
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
