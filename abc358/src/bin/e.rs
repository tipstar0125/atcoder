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
            K: usize,
            C: [usize; 26]
        }

        let MOD = 998244353;

        let mut ans = 0;
        let mut Q: BTreeMap<BTreeMap<usize, usize>, usize> = BTreeMap::new();
        for (i, &c) in C.iter().enumerate() {
            if c > 0 {
                ans += 1;
                let mut mp: BTreeMap<usize, usize> = BTreeMap::new();
                for j in 0..26 {
                    if i == j {
                        mp.insert(j, 1);
                    } else {
                        mp.insert(j, 0);
                    }
                }
                *Q.entry(mp).or_default() += 1;
            }
        }
        for _ in 1..K {
            // for (k, v) in Q.iter() {
            //     eprintln!("{:?} {}", k, v);
            // }
            // eprintln!();
            let mut nQ: BTreeMap<BTreeMap<usize, usize>, usize> = BTreeMap::new();
            for (mp, num) in Q.iter() {
                let mut cnt = 0;
                for (k, v) in mp {
                    if C[*k] > *v {
                        cnt += 1;
                        let mut nmp = mp.clone();
                        *nmp.entry(*k).or_default() += 1;
                        *nQ.entry(nmp).or_default() += num;
                    }
                }
                ans += cnt * num % MOD;
                ans %= MOD;
            }
            Q = nQ;
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
