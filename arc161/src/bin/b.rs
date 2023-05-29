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
            T: usize
        }

        for _ in 0..T {
            input! {
                N: usize
            }

            let mut v = vec![];
            let mut p = 0_usize;
            let mut n = N;
            while n > 0 {
                if n % 2 == 1 {
                    v.push(p);
                }
                p += 1;
                n >>= 1;
            }

            if N < 7 {
                println!("-1");
            } else if N.count_ones() == 3 {
                println!("{}", N);
            } else if N.count_ones() > 3 {
                let mut ans = 0_usize;
                for i in 0..3 {
                    ans += 1 << v[v.len() - i - 1];
                }
                println!("{}", ans);
            } else if N.count_ones() == 1 {
                let mut x = v[0];
                let mut ans = 0_usize;
                for _ in 0..3 {
                    x -= 1;
                    ans += 1 << x;
                }
                println!("{}", ans);
            } else {
                let mut x = v[0];
                if v[0] <= 1 {
                    let mut ans = 0_usize;
                    x = v[1];
                    for _ in 0..3 {
                        x -= 1;
                        ans += 1 << x;
                    }
                    println!("{}", ans);
                } else {
                    let mut ans = 1_usize << v[1];
                    for _ in 0..2 {
                        x -= 1;
                        ans += 1 << x;
                    }
                    println!("{}", ans);
                }
            }
        }
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
