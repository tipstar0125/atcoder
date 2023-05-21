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

const DIJ9: [(usize, usize); 9] = [
    (0, 0),
    (!0, !0),
    (0, !0),
    (1, !0),
    (1, 0),
    (1, 1),
    (0, 1),
    (!0, 1),
    (!0, 0),
];

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            H: usize,
            W: usize,
            S: [Chars; H]
        }

        let snuke = "snuke";
        let snuke: Vec<char> = snuke.chars().collect();
        for i in 0..H {
            for j in 0..W {
                for &(dr, dc) in &DIJ9 {
                    let mut ans = vec![];
                    for (cnt, &x) in snuke.iter().enumerate() {
                        let row = i.wrapping_add(dr.wrapping_mul(cnt));
                        let col = j.wrapping_add(dc.wrapping_mul(cnt));
                        if row >= H || col >= W {
                            break;
                        }
                        if S[row][col] != x {
                            break;
                        } else {
                            ans.push((row, col));
                        }
                    }
                    if ans.len() == 5 {
                        for &(row, col) in &ans {
                            println!("{} {}", row + 1, col + 1);
                        }
                        return;
                    }
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
