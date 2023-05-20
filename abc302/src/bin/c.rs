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
            S: [Chars; N]
        }

        for T in S.iter().permutations(N) {
            let mut ok = true;
            for i in 1..N {
                let t0 = T[i - 1];
                let t1 = T[i];
                let mut cnt = 0;
                for j in 0..M {
                    if t0[j] != t1[j] {
                        cnt += 1;
                    }
                }
                if cnt != 1 {
                    ok = false;
                }
            }
            if ok {
                println!("Yes");
                return;
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
