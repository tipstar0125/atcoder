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

        let mut P = vec![];
        let mut C = vec![];
        let mut F = vec![];
        for _ in 0..N {
            input! {
                p: usize,
                c: usize,
                f: [usize; c]
            }
            P.push(p);
            C.push(c);
            F.push(f);
        }

        for i in 0..N {
            for j in 0..N {
                if i == j {
                    continue;
                }
                if P[i] < P[j] {
                    continue;
                }
                let mut ok = true;
                for &item in &F[i] {
                    if !F[j].contains(&item) {
                        ok = false;
                    }
                }
                if !ok {
                    continue;
                }
                if P[i] > P[j] || F[j].len() > F[i].len() {
                    println!("Yes");
                    return;
                }
            }
        }
        println!("No");
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
