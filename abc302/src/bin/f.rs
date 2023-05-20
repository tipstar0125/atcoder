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
            M: usize,
        }

        let mut S = BTreeSet::new();
        let mut set = BTreeSet::new();

        for _ in 0..N {
            input! {
                A: usize,
                s: [usize; A]
            }
            let s: BTreeSet<usize> = s.iter().cloned().collect();

            if s.contains(&1) {
                set = &set | &s;
            } else {
                S.insert(s);
            }
        }


        let mut ans = 0_usize;
        let mut len = S.len();
        loop {
            if set.contains(&M) {
                println!("{}", ans);
                return;
            }
            ans += 1;

            let mut T = BTreeSet::new();
            let mut ok = vec![false; S.len()];
            for &x in set.clone().iter() {
                for (i, s) in S.clone().iter().enumerate() {
                    if s.contains(&x) {
                        set = &set | s;
                        ok[i] = true;
                    }
                }
            }

            for (i, &o) in ok.iter().enumerate() {
                if o {
                    T.insert(S.iter().nth(i).unwrap().clone());
                }
            }

            S = T;
            if S.len() == len {
                break;
            } else {
                len = S.len();
            }
        }
        println!("-1");
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
