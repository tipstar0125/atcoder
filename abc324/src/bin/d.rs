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
            S: Chars
        }

        let mut S_cnt = vec![0; 10];
        for i in 0..N {
            let n = (S[i] as u8 - b'0') as usize;
            S_cnt[n] += 1;
        }

        let MAX = 1e13_f64.sqrt() as usize + 10;
        let mut ans = 0;
        for i in 0..=MAX {
            let p = i * i;
            let mut ps = p.to_string().chars().collect_vec();
            while ps.len() < N {
                ps.push('0');
            }
            let mut ps_cnt = vec![0; 10];
            for i in 0..ps.len() {
                let n = (ps[i] as u8 - b'0') as usize;
                ps_cnt[n] += 1;
            }
            let mut ok = true;
            for i in 0..10 {
                if ps_cnt[i] != S_cnt[i] {
                    ok = false;
                }
            }
            if ok {
                ans += 1;
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
