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
            V1: isize,
            V2: isize,
            V3: isize,
        }
        for bz in 0..=7 {
            for cx in 0..=14 {
                for cy in 0..=14 {
                    for cz in 0..=14 {
                        let mut mp: BTreeMap<(usize, usize, usize), usize> = BTreeMap::new();
                        for x in 0..7 {
                            for y in 0..7 {
                                for z in 0..7 {
                                    *mp.entry((x, y, z)).or_default() += 1;
                                }
                            }
                        }
                        for x in 0..7 {
                            for y in 0..7 {
                                for z in 0..7 {
                                    *mp.entry((x, y, bz + z)).or_default() += 1;
                                }
                            }
                        }
                        for x in 0..7 {
                            for y in 0..7 {
                                for z in 0..7 {
                                    *mp.entry((cx + x, cy + y, cz + z)).or_default() += 1;
                                }
                            }
                        }
                        let mut ans = vec![0; 4];
                        for (_, v) in mp.iter() {
                            ans[*v] += 1;
                        }
                        if ans[1] == V1 && ans[2] == V2 && ans[3] == V3 {
                            println!("Yes");
                            println!("0 0 0 0 {} {} {} {}", bz, cx, cy, cz);
                            return;
                        }
                    }
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
