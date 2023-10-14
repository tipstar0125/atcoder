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

        let mut v1 = vec![0; T.len()];
        let mut v2 = vec![0; T.len()];

        for s in S.iter() {
            let mut i = 0;
            for si in s {
                if *si == T[i] {
                    i += 1;
                    if i == T.len() {
                        break;
                    }
                }
            }
            if i > 0 {
                v1[i - 1] += 1;
            }
            let mut i = T.len() - 1;
            for si in s.iter().rev() {
                if *si == T[i] {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
            }
            if i < T.len() - 1 {
                v2[i + 1] += 1;
            }
        }
        println!("{:?}", v1);
        println!("{:?}", v2);
        let mut ans = v1[T.len() - 1] * N;
        println!("{}", ans);
        v1.reverse();
        for i in 1..T.len() {
            v2[i] += v2[i - 1];
        }
        v2.reverse();
        // println!("{:?}", v2);
        for i in 1..T.len() {
            ans += v1[i] * v2[i - 1];
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
